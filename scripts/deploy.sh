#!/bin/bash

# System Info Service Deployment Script
# This script deploys the system-info-service to multiple hosts via SSH
# Usage: ./scripts/deploy.sh [--method git|rsync] [--hosts-file path/to/hosts.txt]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default configuration
DEPLOY_METHOD="git"
HOSTS_FILE="scripts/deploy-hosts.txt"
SERVICE_NAME="system-info.service"
GIT_REPO="https://github.com/rpg2014/system-info-service"
GIT_BRANCH="main"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --method)
            DEPLOY_METHOD="$2"
            shift 2
            ;;
        --hosts-file)
            HOSTS_FILE="$2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --method git|rsync    Deployment method (default: git)"
            echo "  --hosts-file PATH     Path to hosts file (default: scripts/deploy-hosts.txt)"
            echo "  --help                Show this help message"
            echo ""
            echo "Hosts file format (one per line):"
            echo "  user@hostname"
            echo "  user@ip.address"
            echo ""
            echo "Example hosts file:"
            echo "  pi@192.168.1.100"
            echo "  pi@raspberrypi.local"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Function to print colored messages
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if hosts file exists
check_hosts_file() {
    if [[ ! -f "$HOSTS_FILE" ]]; then
        log_error "Hosts file not found: $HOSTS_FILE"
        log_info "Creating example hosts file at $HOSTS_FILE"
        cat > "$HOSTS_FILE" << 'EOF'
# System Info Service Deployment Hosts
# Format: user@hostname or user@ip.address
# Example:
# pi@192.168.1.100
# pi@raspberrypi.local
EOF
        log_warning "Please edit $HOSTS_FILE and add your target hosts"
        exit 1
    fi
}

# Function to check if this is first-time install
is_first_install() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    if ssh "$host" "[ -d $remote_dir ]" 2>/dev/null; then
        return 1  # Not first install
    else
        return 0  # First install
    fi
}

# Function to ensure dependencies are installed
ensure_dependencies() {
    local host=$1
    
    log_info "[$host] Checking dependencies..."
    
    # Check for git
    if ! ssh "$host" "command -v git &> /dev/null"; then
        log_info "[$host] Installing git..."
        ssh "$host" "sudo apt update && sudo apt install git -y"
    fi
    
    # Check for cargo/rust
    if ! ssh "$host" "command -v cargo &> /dev/null"; then
        log_info "[$host] Installing Rust..."
        ssh "$host" "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nightly --profile minimal -y"
        ssh "$host" "source \$HOME/.cargo/env"
    fi
    
    log_success "[$host] Dependencies verified"
}

# Function to deploy via git
deploy_git() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    log_info "[$host] Deploying via git..."
    
    if is_first_install "$host"; then
        log_info "[$host] First-time install - cloning repository..."
        ssh "$host" "git clone -b $GIT_BRANCH $GIT_REPO $remote_dir"
    else
        log_info "[$host] Updating existing installation..."
        ssh "$host" "cd $remote_dir && git fetch origin && git reset --hard origin/$GIT_BRANCH"
    fi
}

# Function to deploy via rsync
deploy_rsync() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    log_info "[$host] Deploying via rsync..."
    
    # Ensure remote directory exists
    ssh "$host" "mkdir -p $remote_dir"
    
    # Rsync the code (excluding target directory and git files)
    rsync -avz --delete \
        --exclude 'target/' \
        --exclude '.git/' \
        --exclude '.gitignore' \
        --exclude 'scripts/deploy-hosts.txt' \
        ./ "$host:$remote_dir/"
}

# Function to setup Rocket.toml with secret key
setup_rocket_config() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    log_info "[$host] Setting up Rocket configuration..."
    
    # Check if secret_key already exists in Rocket.toml
    if ssh "$host" "grep -q 'secret_key' $remote_dir/Rocket.toml" 2>/dev/null; then
        log_info "[$host] Rocket.toml already has secret_key"
    else
        log_info "[$host] Generating secret key for Rocket.toml..."
        ssh "$host" "cd $remote_dir && echo 'secret_key = \"'\$(openssl rand -base64 32)'\"' >> Rocket.toml"
    fi
    
    # Copy Rocket.toml to root if needed
    ssh "$host" "sudo cp $remote_dir/Rocket.toml / 2>/dev/null || true"
}

# Function to build the project
build_project() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    log_info "[$host] Building project (this may take a few minutes)..."
    
    # Build in release mode
    if ssh "$host" "cd $remote_dir && source \$HOME/.cargo/env && cargo build --release"; then
        log_success "[$host] Build completed successfully"
    else
        log_error "[$host] Build failed"
        return 1
    fi
    
    # Install the binary
    log_info "[$host] Installing binary..."
    ssh "$host" "cd $remote_dir && source \$HOME/.cargo/env && cargo install --path ."
}

# Function to setup systemd service
setup_service() {
    local host=$1
    local user=$(echo $host | cut -d'@' -f1)
    local remote_dir="/home/$user/system-info-service"
    
    log_info "[$host] Setting up systemd service..."
    
    # Copy service file to systemd directory
    ssh "$host" "sudo cp $remote_dir/system-info.service /etc/systemd/system/"
    
    # Reload systemd
    ssh "$host" "sudo systemctl daemon-reload"
    
    # Enable service
    ssh "$host" "sudo systemctl enable $SERVICE_NAME"
    
    log_success "[$host] Service configured"
}

# Function to restart the service
restart_service() {
    local host=$1
    
    log_info "[$host] Restarting service..."
    
    # Restart the service
    ssh "$host" "sudo systemctl restart $SERVICE_NAME"
    
    # Wait a moment for service to start
    sleep 2
    
    # Check service status
    if ssh "$host" "sudo systemctl is-active --quiet $SERVICE_NAME"; then
        log_success "[$host] Service is running"
        
        # Show brief service status
        ssh "$host" "sudo systemctl status $SERVICE_NAME --no-pager -l" | head -n 15
    else
        log_error "[$host] Service failed to start"
        log_error "[$host] Recent logs:"
        ssh "$host" "sudo journalctl -u $SERVICE_NAME -n 30 --no-pager"
        return 1
    fi
}

# Function to deploy to a single host
deploy_to_host() {
    local host=$1
    
    echo ""
    log_info "=========================================="
    log_info "Deploying to: $host"
    log_info "=========================================="
    
    # Test SSH connection
    if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "$host" "echo 'SSH connection successful'" > /dev/null 2>&1; then
        log_error "[$host] Cannot connect via SSH. Skipping..."
        log_warning "[$host] Make sure SSH key authentication is set up"
        return 1
    fi
    
    # Ensure dependencies are installed
    ensure_dependencies "$host" || return 1
    
    # Deploy code
    if [[ "$DEPLOY_METHOD" == "git" ]]; then
        deploy_git "$host" || return 1
    else
        deploy_rsync "$host" || return 1
    fi
    
    # Setup Rocket configuration
    setup_rocket_config "$host" || return 1
    
    # Build project
    build_project "$host" || return 1
    
    # Setup systemd service
    setup_service "$host" || return 1
    
    # Restart service
    restart_service "$host" || return 1
    
    log_success "[$host] Deployment completed successfully!"
    
    return 0
}

# Main deployment logic
main() {
    log_info "System Info Service Deployment Script"
    log_info "Deployment method: $DEPLOY_METHOD"
    log_info "Hosts file: $HOSTS_FILE"
    echo ""
    
    # Check if hosts file exists
    check_hosts_file
    
    # Read hosts from file (skip comments and empty lines)
    hosts=()
    while IFS= read -r line; do
        # Skip comments and empty lines
        [[ "$line" =~ ^#.*$ ]] && continue
        [[ -z "$line" ]] && continue
        hosts+=("$line")
    done < "$HOSTS_FILE"
    
    if [[ ${#hosts[@]} -eq 0 ]]; then
        log_error "No hosts found in $HOSTS_FILE"
        exit 1
    fi
    
    log_info "Found ${#hosts[@]} host(s) to deploy to"
    
    # Deploy to each host
    success_count=0
    fail_count=0
    
    for host in "${hosts[@]}"; do
        if deploy_to_host "$host"; then
            ((success_count++))
        else
            ((fail_count++))
        fi
    done
    
    # Summary
    echo ""
    log_info "=========================================="
    log_info "Deployment Summary"
    log_info "=========================================="
    log_success "Successful deployments: $success_count"
    if [[ $fail_count -gt 0 ]]; then
        log_error "Failed deployments: $fail_count"
    fi
    
    if [[ $fail_count -eq 0 ]]; then
        log_success "All deployments completed successfully!"
        exit 0
    else
        log_warning "Some deployments failed. Please check the logs above."
        exit 1
    fi
}

# Run main function
main