# Deployment Guide

This guide explains how to deploy the system-info-service to multiple hosts using the automated deployment script.

## Prerequisites

### Local Machine (Deployment Host)
- Bash shell
- SSH client
- rsync (if using rsync deployment method)
- SSH key authentication configured for target hosts

### Target Hosts (Raspberry Pis)
- SSH server running
- sudo access for the deployment user
- Internet connection (for git deployment method)

## Setup

### 1. Configure SSH Key Authentication

Ensure you can SSH into your target hosts without a password:

```bash
# Generate SSH key if you don't have one
ssh-keygen -t ed25519

# Copy your public key to each target host
ssh-copy-id pi@192.168.1.100
ssh-copy-id pi@raspberrypi.local
```

### 2. Create Hosts File

Copy the example hosts file and add your target hosts:

```bash
cp scripts/deploy-hosts.txt.example scripts/deploy-hosts.txt
```

Edit `scripts/deploy-hosts.txt` and add your hosts (one per line):

```
pi@192.168.1.100
pi@192.168.1.101
pi@raspberrypi-livingroom.local
pi@raspberrypi-bedroom.local
```

## Deployment Methods

### Git Method (Recommended)

Pulls the latest code from GitHub on each target host:

```bash
./scripts/deploy.sh --method git
```

**Advantages:**
- Ensures all hosts get the exact same code from the repository
- Smaller network transfer
- Good for production deployments

**Requirements:**
- Target hosts must have internet access
- Code must be committed and pushed to GitHub

### Rsync Method

Copies code from your local machine to target hosts:

```bash
./scripts/deploy.sh --method rsync
```

**Advantages:**
- Works without internet on target hosts
- Can deploy uncommitted changes
- Good for testing and development

**Requirements:**
- rsync installed on local machine

## Usage

### Basic Deployment

Deploy to all hosts in the default hosts file using git:

```bash
./scripts/deploy.sh
```

### Custom Hosts File

Deploy using a different hosts file:

```bash
./scripts/deploy.sh --hosts-file /path/to/custom-hosts.txt
```

### Using Rsync

Deploy using rsync instead of git:

```bash
./scripts/deploy.sh --method rsync
```

### Help

View all available options:

```bash
./scripts/deploy.sh --help
```

## What the Script Does

For each target host, the deployment script:

1. **Tests SSH Connection** - Verifies it can connect to the host
2. **Installs Dependencies** - Ensures git and Rust/Cargo are installed
3. **Deploys Code** - Either clones/pulls from git or rsyncs from local machine
4. **Configures Rocket** - Generates secret key if needed
5. **Builds Project** - Compiles the release binary (takes a few minutes)
6. **Installs Binary** - Installs to `~/.cargo/bin/`
7. **Configures Service** - Sets up systemd service
8. **Restarts Service** - Restarts the service and verifies it's running

## Deployment Output

The script provides colored output:
- 🔵 **BLUE** - Informational messages
- 🟢 **GREEN** - Success messages
- 🟡 **YELLOW** - Warnings
- 🔴 **RED** - Errors

Example output:
```
[INFO] System Info Service Deployment Script
[INFO] Deployment method: git
[INFO] Hosts file: scripts/deploy-hosts.txt

[INFO] Found 2 host(s) to deploy to

[INFO] ==========================================
[INFO] Deploying to: pi@192.168.1.100
[INFO] ==========================================
[INFO] [pi@192.168.1.100] Checking dependencies...
[SUCCESS] [pi@192.168.1.100] Dependencies verified
[INFO] [pi@192.168.1.100] Deploying via git...
[INFO] [pi@192.168.1.100] Updating existing installation...
[INFO] [pi@192.168.1.100] Building project (this may take a few minutes)...
[SUCCESS] [pi@192.168.1.100] Build completed successfully
[SUCCESS] [pi@192.168.1.100] Service is running
[SUCCESS] [pi@192.168.1.100] Deployment completed successfully!

[INFO] ==========================================
[INFO] Deployment Summary
[INFO] ==========================================
[SUCCESS] Successful deployments: 2
[SUCCESS] All deployments completed successfully!
```

## Troubleshooting

### SSH Connection Failed

**Error:** `Cannot connect via SSH. Skipping...`

**Solution:**
- Verify the host is reachable: `ping hostname`
- Test SSH manually: `ssh user@hostname`
- Ensure SSH key authentication is set up: `ssh-copy-id user@hostname`

### Build Failed

**Error:** `Build failed`

**Solution:**
- SSH into the host and check disk space: `df -h`
- Check Rust installation: `cargo --version`
- Try building manually: `cd ~/system-info-service && cargo build --release`

### Service Failed to Start

**Error:** `Service failed to start`

**Solution:**
- Check service logs: `sudo journalctl -u system-info.service -n 50`
- Verify the binary exists: `ls -la ~/.cargo/bin/system_info_service`
- Check Rocket.toml configuration: `cat ~/system-info-service/Rocket.toml`

### Permission Denied

**Error:** Permission errors during deployment

**Solution:**
- Ensure the user has sudo access
- Check file permissions in the project directory
- Verify systemd service file permissions

## Manual Verification

After deployment, you can verify the service on each host:

```bash
# Check service status
ssh pi@hostname "sudo systemctl status system-info.service"

# View recent logs
ssh pi@hostname "sudo journalctl -u system-info.service -n 50"

# Test the API
curl http://hostname:8000/api/health
curl http://hostname:8000/api/system/all
```

## Rollback

If a deployment fails or causes issues, you can rollback:

```bash
# SSH into the affected host
ssh pi@hostname

# Go to the project directory
cd ~/system-info-service

# Checkout previous version
git log --oneline  # Find the commit hash
git reset --hard <previous-commit-hash>

# Rebuild and restart
cargo build --release
cargo install --path .
sudo systemctl restart system-info.service
```

## Best Practices

1. **Test First** - Deploy to a single test host before deploying to all hosts
2. **Commit Changes** - Always commit and push changes before deploying with git method
3. **Monitor Logs** - Watch service logs after deployment to catch issues early
4. **Backup Configuration** - Keep backups of Rocket.toml and service files
5. **Staged Rollout** - Deploy to hosts in batches rather than all at once
6. **Version Tags** - Use git tags for releases to make rollbacks easier

## Advanced Usage

### Deploy to Specific Hosts

Create a temporary hosts file with just the hosts you want to deploy to:

```bash
echo "pi@192.168.1.100" > /tmp/single-host.txt
./scripts/deploy.sh --hosts-file /tmp/single-host.txt
```

### Parallel Deployment

For faster deployment to many hosts, you can run multiple deployments in parallel:

```bash
# Split hosts into multiple files
split -l 5 scripts/deploy-hosts.txt /tmp/hosts-

# Deploy to each batch in parallel
for file in /tmp/hosts-*; do
    ./scripts/deploy.sh --hosts-file "$file" &
done
wait
```

## Support

For issues or questions:
- Check the troubleshooting section above
- Review service logs: `sudo journalctl -u system-info.service`
- Open an issue on GitHub