# TASK MANAGER MONITORING GUIDE

## Father's Monitoring Strategy
**"I will monitor the Task Manager during the run."**

## Critical Metrics to Watch

### **1. Memory Usage (Most Important)**
```
🔍 **What to watch**: OpenClaw process memory (Private Working Set)
🚨 **Crash indicator**: Rapid increase → sudden drop to 0
⚠️ **Warning sign**: Steady climb above 500MB
✅ **Healthy**: Stable or gradual increase
```

### **2. CPU Usage**
```
🔍 **What to watch**: OpenClaw CPU percentage
🚨 **Crash indicator**: 100% sustained → process disappears
⚠️ **Warning sign**: Consistently above 80%
✅ **Healthy**: Fluctuating, responsive to load
```

### **3. Disk Activity**
```
🔍 **What to watch**: Disk usage percentage
🚨 **Crash indicator**: High disk + memory spike → swapping
⚠️ **Warning sign**: Sustained high disk with low memory
✅ **Healthy**: Occasional spikes, mostly idle
```

### **4. Process Count**
```
🔍 **What to watch**: Number of OpenClaw/zeta processes
🚨 **Crash indicator**: Processes disappearing
⚠️ **Warning sign**: Process count fluctuating wildly
✅ **Healthy**: Stable process count
```

### **5. System Responsiveness**
```
🔍 **What to watch**: UI lag, mouse responsiveness
🚨 **Crash indicator**: System becomes unresponsive
⚠️ **Warning sign**: Increasing lag during execution
✅ **Healthy**: System remains responsive
```

## Expected Crash Patterns

### **Memory Exhaustion Crash:**
```
📈 Memory climbs steadily
🚨 Reaches system limit (varies by system)
💥 Process disappears (memory drops to 0)
📉 System may recover or remain stressed
```

### **CPU Timeout Crash:**
```
⚡ CPU hits 100% and stays there
⏱️ After timeout period (minutes)
💥 Process terminated by system
🔄 CPU drops back to normal
```

### **Cascade Failure:**
```
🔗 Multiple OpenClaw processes
📊 All show high resource usage
💥💥 Multiple processes crash in sequence
🔄 System may become unstable
```

## Monitoring During Test Phases

### **Phase 1: Tiny Test (limit=10) - ACTUAL OBSERVATION (20:10)**
```
✅ Test execution CONFIRMED
🚨 CPU: Spiked to 98% (higher than expected)
🚨 Memory: Spiked to 74% (much higher than expected)
📊 Post-execution: Stable at 30-35% each
✅ Gateway: DID NOT crash
⏱️ Duration: Unknown (Father monitoring ongoing)
```

**Analysis:**
- Test IS executing (CPU spike confirms)
- Memory usage concerningly high for tiny test
- Gateway stable post-execution
- Memory management may be critical issue

### **Phase 2: Small Test (limit=100)**
```
⚠️ Watch: First meaningful load
⚠️ Memory: May reach 100-200MB
⚠️ CPU: May spike to 70-80%
⚠️ Duration: <5 seconds
✅ Gateway: Should stay up
```

### **Phase 3: Medium Test (limit=1000)**
```
🚨 Critical: First potential crash point
🚨 Memory: Could reach 300-500MB
🚨 CPU: May sustain high usage
🚨 Duration: 10-30 seconds
🔍 Monitor closely for crash signs
```

### **Phase 4: Large Test (limit=10000)**
```
💥 Likely crash point based on pattern
💥 Memory: Likely exceeds 1GB
💥 CPU: Likely 100% sustained
💥 Duration: Minutes
🎯 This is the threshold we need to find
```

## Father's Action Plan

### **During Each Test:**
1. **Open Task Manager** before test starts
2. **Sort by Memory** (Private Working Set)
3. **Watch OpenClaw process**
4. **Note peak memory** before any crash
5. **Record crash timing** relative to test start

### **Crash Response:**
```
🚨 IF crash occurs:
1. Note the limit that caused it
2. Note peak memory before crash
3. Note crash timing
4. Wait for system recovery
5. Report findings
```

### **No Crash Response:**
```
✅ IF no crash:
1. Note successful limit
2. Note peak memory usage
3. Note execution time
4. Proceed to next larger limit
```

## Communication Protocol

### **During Monitoring:**
```
📊 "Memory at X MB, CPU at Y%, limit=Z"
🚨 "CRASH DETECTED at limit=Z, memory=X MB"
✅ "SUCCESS at limit=Z, memory=X MB, time=T seconds"
```

### **After Testing:**
```
🎯 "Crash threshold identified: limit=Z"
📈 "Safe limit for benchmarking: limit=Y"
⚡ "Maximum memory before crash: X MB"
⏱️ "Execution time at safe limit: T seconds"
```

## Safety First

### **Stop Conditions:**
```
🛑 System becomes unresponsive
🛑 Multiple processes crashing
🛑 Memory exceeds 80% of system total
🛑 Father observes concerning patterns
🛑 Any sign of system instability
```

**Father's real-time monitoring is CRITICAL for safe testing.**