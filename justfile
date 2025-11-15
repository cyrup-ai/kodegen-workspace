# Justfile for KODEGEN.ᴀɪ workspace

# Run cargo check and clippy on all Rust projects, saving results to task/ (only on failure)
check:
    #!/usr/bin/env bash
    # Note: Don't use 'set -e' so we continue even if individual projects fail
    
    # Create task directory if it doesn't exist
    mkdir -p task
    
    # List of all Rust projects in the workspace
    projects=(
        "cylo"
        "kodegen"
        "kodegen-bundler-autoconfig"
        "kodegen-bundler-release"
        "kodegen-bundler-sign"
        "kodegen-candle-agent"
        "kodegen-claude-agent"
        "kodegen-mcp-client"
        "kodegen-mcp-schema"
        "kodegen-mcp-tool"
        "kodegen-server-http"
        "kodegen-simd"
        "kodegen-tools-browser"
        "kodegen-tools-citescrape"
        "kodegen-tools-config"
        "kodegen-tools-database"
        "kodegen-tools-filesystem"
        "kodegen-tools-git"
        "kodegen-tools-github"
        "kodegen-tools-introspection"
        "kodegen-tools-process"
        "kodegen-tools-prompt"
        "kodegen-tools-reasoner"
        "kodegen-tools-sequential-thinking"
        "kodegen-tools-terminal"
        "kodegen-utils"
        "kodegend"
    )
    
    echo "Running cargo check and clippy on all projects..."
    echo "Failed projects will be saved to task/"
    echo ""
    
    failed_projects=()
    succeeded_projects=()
    
    for project in "${projects[@]}"; do
        if [ -d "$project" ] && [ -f "$project/Cargo.toml" ]; then
            echo "Checking $project..."
            output_file="task/${project}.txt"
            
            # Delete old file if it exists (ensures only failures generate files)
            rm -f "$output_file"
            
            # Capture cargo check output (treat warnings as errors)
            check_output=$(cd "$project" && RUSTFLAGS="-D warnings" cargo check 2>&1)
            check_exit=$?
            
            # Capture cargo clippy output (treat warnings as errors)
            clippy_output=$(cd "$project" && cargo clippy -- -D warnings 2>&1)
            clippy_exit=$?
            
            # Determine status
            if [ $check_exit -eq 0 ]; then
                check_status="✓"
            else
                check_status="✗"
            fi
            
            if [ $clippy_exit -eq 0 ]; then
                clippy_status="✓"
            else
                clippy_status="✗"
            fi
            
            # Only write file if either command failed
            if [ $check_exit -ne 0 ] || [ $clippy_exit -ne 0 ]; then
                # Create output file with header
                echo "===============================================" > "$output_file"
                echo "Project: $project" >> "$output_file"
                echo "Date: $(date)" >> "$output_file"
                echo "===============================================" >> "$output_file"
                echo "" >> "$output_file"
                
                # Write cargo check results
                echo "--- CARGO CHECK ---" >> "$output_file"
                echo "" >> "$output_file"
                echo "$check_output" >> "$output_file"
                echo "" >> "$output_file"
                
                # Write cargo clippy results
                echo "--- CARGO CLIPPY ---" >> "$output_file"
                echo "" >> "$output_file"
                echo "$clippy_output" >> "$output_file"
                echo "" >> "$output_file"
                
                # Add footer
                echo "===============================================" >> "$output_file"
                echo "Finished checking $project" >> "$output_file"
                echo "===============================================" >> "$output_file"
                
                echo "  ✗ Failed (check: $check_status, clippy: $clippy_status) - Results saved to $output_file"
                failed_projects+=("$project")
            else
                echo "  ✓ Passed"
                succeeded_projects+=("$project")
            fi
        else
            echo "  ⚠ Skipping $project (not found or no Cargo.toml)"
        fi
    done
    
    echo ""
    echo "==============================================="
    echo "Summary"
    echo "==============================================="
    echo "Total projects: ${#projects[@]}"
    echo "Succeeded: ${#succeeded_projects[@]}"
    echo "Failed: ${#failed_projects[@]}"
    
    if [ ${#failed_projects[@]} -gt 0 ]; then
        echo ""
        echo "Failed projects:"
        for project in "${failed_projects[@]}"; do
            echo "  - $project"
        done
        echo ""
        echo "Error details saved in task/"
    else
        echo ""
        echo "All projects passed! No output files generated."
    fi

# List all projects
list-projects:
    @echo "Rust projects in workspace:"
    @find . -maxdepth 2 -name "Cargo.toml" -not -path "*/target/*" | sed 's|./||' | sed 's|/Cargo.toml||' | sort

# List all projects
mcp:
    @pkill -f kodegen-browser || true
    @pkill -f kodegen-citescrape || true
    @pkill -f kodegen-claude-agent || true
    @pkill -f kodegen-config || true
    @pkill -f kodegen-database || true
    @pkill -f kodegen-filesystem || true
    @pkill -f kodegen-git || true
    @pkill -f kodegen-github || true
    @pkill -f kodegen-introspection || true
    @pkill -f kodegen-process || true
    @pkill -f kodegen-prompt || true
    @pkill -f kodegen-reasoner || true
    @pkill -f kodegen-sequential-thinking || true
    @pkill -f kodegen-terminal || true
    @pkill -f kodegen-candle-agent || true
    @rm -rf ./packages/tmp/mcp
    @mkdir -p ./packages/tmp/mcp
    @find /Users/davidmaple/kodegen-workspace -name "Cargo.lock" -type f -delete
    @nohup sh -c 'cd ./packages/kodegen && cargo clean && cargo install --path .' > ./packages/tmp/mcp/kodegen.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-browser && cargo clean && cargo install --path . --force && kodegen-browser --http 127.0.0.1:30438' > ./packages/tmp/mcp/kodegen-browser.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-citescrape && cargo clean && cargo install --path . --force && kodegen-citescrape --http 127.0.0.1:30439' > ./packages/tmp/mcp/kodegen-citescrape.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-claude-agent && cargo clean && cargo install --path . --force && kodegen-claude-agent --http 127.0.0.1:30440' > ./packages/tmp/mcp/kodegen-claude-agent.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-config && cargo clean && cargo install --path . --force && kodegen-config --http 127.0.0.1:30441' > ./packages/tmp/mcp/kodegen-config.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-database && cargo clean && cargo install --path . --force && kodegen-database --http 127.0.0.1:30442' > ./packages/tmp/mcp/kodegen-database.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-filesystem && cargo clean && cargo install --path . --force && kodegen-filesystem --http 127.0.0.1:30443' > ./packages/tmp/mcp/kodegen-filesystem.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-git && cargo clean && cargo install --path . --force && kodegen-git --http 127.0.0.1:30444' > ./packages/tmp/mcp/kodegen-git.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-github && cargo clean && cargo install --path . --force && kodegen-github --http 127.0.0.1:30445' > ./packages/tmp/mcp/kodegen-github.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-introspection && cargo clean && cargo install --path . --force && kodegen-introspection --http 127.0.0.1:30446' > ./packages/tmp/mcp/kodegen-introspection.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-process && cargo clean && cargo install --path . --force && kodegen-process --http 127.0.0.1:30447' > ./packages/tmp/mcp/kodegen-process.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-prompt && cargo clean && cargo install --path . --force && kodegen-prompt --http 127.0.0.1:30448' > ./packages/tmp/mcp/kodegen-prompt.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-reasoner && cargo clean && cargo install --path . --force && kodegen-reasoner --http 127.0.0.1:30449' > ./packages/tmp/mcp/kodegen-reasoner.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-sequential-thinking && cargo clean && cargo install --path . --force && kodegen-sequential-thinking --http 127.0.0.1:30450' > ./packages/tmp/mcp/kodegen-sequential-thinking.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-terminal && cargo clean && cargo install --path . --force && kodegen-terminal --http 127.0.0.1:30451' > ./packages/tmp/mcp/kodegen-terminal.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-candle-agent && cargo clean && cargo install --path . --force && kodegen-candle-agent --http 127.0.0.1:30452' > ./packages/tmp/mcp/kodegen-candle-agent.log 2>&1 &
    @tail -F ./packages/tmp/mcp/*.log
