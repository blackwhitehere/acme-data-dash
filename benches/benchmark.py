#!/usr/bin/env python3
import argparse
import subprocess
import time
import statistics
from pathlib import Path

# Configuration
SCRIPT_DIR = Path(__file__).parent.absolute()
WORKSPACE_ROOT = SCRIPT_DIR.parent
RUST_BIN = WORKSPACE_ROOT / "target" / "release" / "acme-rust-template"

# Colors
class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'

def log_info(msg):
    print(f"{Colors.BLUE}[INFO]{Colors.NC} {msg}")

def log_success(msg):
    print(f"{Colors.GREEN}[SUCCESS]{Colors.NC} {msg}")

def log_warning(msg):
    print(f"{Colors.YELLOW}[WARNING]{Colors.NC} {msg}")

def log_error(msg):
    print(f"{Colors.RED}[ERROR]{Colors.NC} {msg}")

def run_command(cmd, cwd=None, capture_output=True):
    try:
        result = subprocess.run(
            cmd, 
            cwd=cwd, 
            shell=True, 
            check=False, 
            stdout=subprocess.PIPE if capture_output else None, 
            stderr=subprocess.PIPE if capture_output else None,
            text=True
        )
        return result
    except Exception:
        return None

def benchmark_func(name, cmd_func, runs=5):
    log_info(f"Benchmarking: {name} ({runs} runs)")
    times = []
    
    for _ in range(runs):
        start = time.perf_counter_ns()
        cmd_func()
        end = time.perf_counter_ns()
        elapsed_ms = (end - start) / 1_000_000
        times.append(elapsed_ms)
    
    avg = statistics.mean(times)
    median = statistics.median(times)
    min_val = min(times)
    max_val = max(times)
    
    return {
        "name": name,
        "avg": avg,
        "median": median,
        "min": min_val,
        "max": max_val
    }

def main():
    parser = argparse.ArgumentParser(description="Acme Rust Template Benchmark Tool", formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--runs", type=int, default=5, help="Number of benchmark runs")
    
    args = parser.parse_args()

    if not RUST_BIN.exists():
        log_info("Building release binary...")
        run_command("cargo build --release", cwd=WORKSPACE_ROOT)

    print("\n" + "="*60)
    print("                    BENCHMARK RESULTS")
    print("="*60 + "\n")
    
    results = []
    
    # Rust Run
    def run_rust():
        run_command(f"'{RUST_BIN}'")
    
    results.append(benchmark_func("Rust Execution", run_rust, args.runs))

    # Report
    print(f"{'Method':<25} {'Avg(ms)':>10} {'Median(ms)':>10} {'Min(ms)':>10} {'Max(ms)':>10}")
    print("-" * 65)
    for r in results:
        print(f"{r['name']:<25} {r['avg']:>10.2f} {r['median']:>10.2f} {r['min']:>10.2f} {r['max']:>10.2f}")
    print("="*60 + "\n")

    output_data = {
        "results": results,
        "stats": {}
    }

    # Save to JSON
    import json
    json_output = SCRIPT_DIR / "benchmark_results.json"
    with open(json_output, "w") as f:
        json.dump(output_data, f, indent=2)
    log_success(f"Results saved to {json_output}")

if __name__ == "__main__":
    main()
