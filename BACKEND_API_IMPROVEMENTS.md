# Fleet Monitor Backend API Improvements

## Overview
This document outlines proposed backend API improvements to enhance performance, reduce network overhead, and improve user experience for the Fleet Monitor application.

## Smithy API Contract

```smithy
$version: "2"

namespace com.fleetmonitor.api

use aws.protocols#restJson1

@restJson1
service FleetMonitorService {
    version: "2024-01-01"
    operations: [
        GetSystemHealth,
        GetSystemMetrics,
        GetSystemAll,
        StreamSystemMetrics
    ]
}

// Health Check - Lightweight endpoint for status checking
@http(method: "GET", uri: "/api/health")
@readonly
operation GetSystemHealth {
    output: SystemHealthOutput
}

structure SystemHealthOutput {
    @required
    status: String
    
    @required
    timestamp: Timestamp
    
    uptime: String
    hostname: String
}

// Individual Metrics - Keep existing endpoints for compatibility
@http(method: "GET", uri: "/api/system/{metric}")
@readonly
operation GetSystemMetrics {
    input: SystemMetricsInput
    output: SystemMetricsOutput
}

structure SystemMetricsInput {
    @httpLabel
    @required
    metric: String
}

union SystemMetricsOutput {
    uptime: String
    hostname: String
    cpuTemp: Float
    loadAverage: LoadAverageData
    networks: NetworkData
    netStats: NetworkStatsData
    cpuAverage: CpuAverageData
}

// Batch Endpoint - Get all metrics in one request
@http(method: "GET", uri: "/api/system/all")
@readonly
operation GetSystemAll {
    input: SystemAllInput
    output: SystemAllOutput
}

structure SystemAllInput {
    // Optional filter for specific metrics
    metrics: StringList
    
    // Include historical data points
    includeHistory: Boolean = false
    
    // History duration in seconds
    historyDuration: Integer = 300
}

structure SystemAllOutput {
    @required
    timestamp: Timestamp
    
    @required
    hostname: String
    
    @required
    uptime: String
    
    cpuTemp: Float
    loadAverage: LoadAverageData
    networks: NetworkData
    netStats: NetworkStatsData
    cpuAverage: CpuAverageData
    
    // Historical data if requested
    history: HistoricalData
}

// WebSocket Streaming - Real-time updates
@http(method: "GET", uri: "/api/stream/metrics")
operation StreamSystemMetrics {
    input: StreamMetricsInput
    output: StreamMetricsOutput
}

structure StreamMetricsInput {
    // Metrics to stream
    metrics: StringList
    
    // Update interval in seconds
    interval: Integer = 5
}

structure StreamMetricsOutput {
    @required
    timestamp: Timestamp
    
    @required
    type: String // "update", "error", "heartbeat"
    
    data: SystemAllOutput
    error: String
}

// Data Structures
structure LoadAverageData {
    @required
    one: String
    
    @required
    five: String
    
    @required
    fifteen: String
}

structure NetworkData {
    @required
    networks: NetworkDetailsList
}

list NetworkDetailsList {
    member: NetworkDetails
}

structure NetworkDetails {
    @required
    name: String
    
    @required
    addrs: NetworkAddressList
}

list NetworkAddressList {
    member: NetworkAddress
}

structure NetworkAddress {
    @required
    addr: StringMap
}

map StringMap {
    key: String
    value: String
}

structure NetworkStatsData {
    One: NetworkStats
    List: NetworkStatsList
}

list NetworkStatsList {
    member: NetworkStats
}

structure NetworkStats {
    @required
    network_name: String
    
    @required
    rx_bytes: Long
    
    @required
    tx_bytes: Long
    
    @required
    rx_packets: Long
    
    @required
    tx_packets: Long
    
    @required
    rx_errors: Long
    
    @required
    tx_errors: Long
}

structure CpuAverageData {
    @required
    user: Float
    
    @required
    nice: Float
    
    @required
    system: Float
    
    @required
    interrupt: Float
    
    @required
    idle: Float
}

structure HistoricalData {
    cpuTemp: TimeSeriesData
    loadAverage: TimeSeriesData
    cpuUsage: TimeSeriesData
}

structure TimeSeriesData {
    @required
    timestamps: TimestampList
    
    @required
    values: FloatList
}

list TimestampList {
    member: Timestamp
}

list FloatList {
    member: Float
}

list StringList {
    member: String
}
```

## Implementation Priority

### Phase 1: Quick Wins
1. **Health Check Endpoint** (`/api/health`)
   - Lightweight status check
   - Returns basic info: status, hostname, uptime
   - Use for connection testing

2. **Batch Endpoint** (`/api/system/all`)
   - Combine all existing endpoints into one
   - Reduces 6+ requests to 1 request per device
   - Massive performance improvement

### Phase 2: Real-time Features
3. **WebSocket Streaming** (`/api/stream/metrics`)
   - Replace polling with real-time updates
   - Configurable update intervals
   - Heartbeat mechanism for connection health

4. **Historical Data**
   - Store metrics in lightweight database (SQLite)
   - Enable longer time ranges for graphs
   - Optional parameter in `/api/system/all`

### Phase 3: Production Ready
5. **Error Handling**
   - Proper HTTP status codes
   - Structured error responses
   - Graceful degradation

6. **Security & Performance**
   - CORS headers
   - Compression (gzip)
   - Rate limiting
   - Optional authentication

## Example Responses

### Health Check
```json
GET /api/health
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "uptime": "5 days, 3 hours",
  "hostname": "pi-livingroom"
}
```

### Batch Metrics
```json
GET /api/system/all
{
  "timestamp": "2024-01-01T12:00:00Z",
  "hostname": "pi-livingroom",
  "uptime": "5 days, 3 hours",
  "cpuTemp": 45.2,
  "loadAverage": {
    "one": "0.15",
    "five": "0.23",
    "fifteen": "0.18"
  },
  "networks": { /* existing structure */ },
  "netStats": { /* existing structure */ },
  "cpuAverage": { /* existing structure */ }
}
```

### WebSocket Stream
```json
{
  "timestamp": "2024-01-01T12:00:00Z",
  "type": "update",
  "data": {
    "cpuTemp": 45.3,
    "loadAverage": { "one": "0.16", "five": "0.23", "fifteen": "0.18" }
  }
}
```

## Migration Strategy

1. **Backward Compatibility**: Keep existing endpoints working
2. **Feature Flags**: Enable new endpoints gradually
3. **Client Updates**: Update frontend to use batch endpoint first
4. **WebSocket**: Add streaming as enhancement
5. **Deprecation**: Eventually deprecate individual endpoints

## Benefits

- **Performance**: 6x fewer HTTP requests
- **Real-time**: Instant updates via WebSocket
- **Reliability**: Health checks for better error handling
- **Scalability**: Reduced server load
- **User Experience**: Faster loading, live updates
