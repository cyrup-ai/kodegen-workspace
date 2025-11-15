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

# Convert workspace dependencies to local path dependencies (version + path)
dep-local:
    #!/usr/bin/env python3
    import os
    import re
    from pathlib import Path
    from typing import Dict, Tuple

    def find_packages() -> Dict[str, Tuple[Path, str]]:
        """Find all packages and their versions."""
        packages = {}
        packages_dir = Path("packages")

        if not packages_dir.exists():
            return packages

        for cargo_toml in packages_dir.rglob("Cargo.toml"):
            # Skip target and tmp directories - tmp is ONLY for docs, never for dependencies!
            if "target" in cargo_toml.parts or "tmp" in cargo_toml.parts:
                continue

            pkg_name = None
            pkg_version = None

            with open(cargo_toml, 'r') as f:
                for line in f:
                    if pkg_name is None:
                        match = re.match(r'^\s*name\s*=\s*"([^"]+)"', line)
                        if match:
                            pkg_name = match.group(1)

                    if pkg_version is None:
                        match = re.match(r'^\s*version\s*=\s*"([^"]+)"', line)
                        if match:
                            pkg_version = match.group(1)

                    if pkg_name and pkg_version:
                        break

            if pkg_name:
                normalized_name = pkg_name.replace("-", "_")
                packages[normalized_name] = (cargo_toml.parent, pkg_version or "0.0.0")

        return packages

    def calculate_relative_path(from_path: Path, to_path: Path) -> str:
        """Calculate relative path between packages."""
        rel = os.path.relpath(to_path, from_path)
        return rel.replace(os.sep, "/")

    def process_file(cargo_toml: Path, packages: Dict[str, Tuple[Path, str]], pkg_path: Path) -> bool:
        """Process a Cargo.toml file. Returns True if modified."""
        with open(cargo_toml, 'r') as f:
            content = f.read()

        original_content = content
        lines = content.split('\n')
        new_lines = []
        modified = False

        for line in lines:
            # Match dependency line: dep = { ... }
            match = re.match(r'^(\s*)([a-zA-Z0-9_-]+)\s*=\s*\{([^}]+)\}(.*)$', line)

            if not match:
                new_lines.append(line)
                continue

            indent = match.group(1)
            dep_name = match.group(2)
            attrs_str = match.group(3)
            rest = match.group(4).lstrip('}')  # Remove any extra closing braces

            normalized_dep = dep_name.replace("-", "_")

            # Only process workspace dependencies
            if normalized_dep not in packages:
                new_lines.append(line)
                continue

            dep_pkg_path, dep_version = packages[normalized_dep]

            # Parse existing attributes
            has_version = re.search(r'version\s*=\s*"([^"]+)"', attrs_str)
            has_path = re.search(r'path\s*=\s*"([^"]+)"', attrs_str)

            # If already has both version and path, skip
            if has_version and has_path:
                new_lines.append(line)
                continue

            # Extract all attributes
            attrs_parts = []

            # Get version (existing or from target package)
            if has_version:
                version = has_version.group(1)
            else:
                version = dep_version
                modified = True

            attrs_parts.append(f'version = "{version}"')

            # Get or calculate path
            if has_path:
                path_val = has_path.group(1)
            else:
                path_val = calculate_relative_path(pkg_path, dep_pkg_path)
                modified = True

            attrs_parts.append(f'path = "{path_val}"')

            # Preserve other attributes (features, optional, etc.)
            other_attrs = []
            for attr_match in re.finditer(r'(features\s*=\s*\[[^\]]*\]|optional\s*=\s*\w+|default-features\s*=\s*\w+)', attrs_str):
                other_attrs.append(attr_match.group(0))

            attrs_parts.extend(other_attrs)

            new_line = indent + dep_name + ' = { ' + ', '.join(attrs_parts) + ' }' + rest
            new_lines.append(new_line)

        if modified:
            with open(cargo_toml, 'w') as f:
                f.write('\n'.join(new_lines))

        return modified

    # Main execution
    packages = find_packages()
    updated_count = 0

    print("Converting to local path dependencies (version + path)...")
    print(f"Found {len(packages)} workspace packages")
    print()

    for pkg_name, (pkg_path, _) in sorted(packages.items()):
        cargo_toml = pkg_path / "Cargo.toml"

        if process_file(cargo_toml, packages, pkg_path):
            print(f"  ✓ {pkg_path.name}")
            updated_count += 1

    print()
    if updated_count == 0:
        print("All dependencies already have local paths with versions")
    else:
        print(f"Updated {updated_count} package(s)")

# Remove local path dependencies (keep version only for published dependencies)
dep-published:
    #!/usr/bin/env python3
    import os
    import re
    from pathlib import Path
    from typing import Dict, Tuple

    def find_packages() -> Dict[str, Tuple[Path, str]]:
        """Find all packages and their versions."""
        packages = {}
        packages_dir = Path("packages")

        if not packages_dir.exists():
            return packages

        for cargo_toml in packages_dir.rglob("Cargo.toml"):
            # Skip target and tmp directories - tmp is ONLY for docs, never for dependencies!
            if "target" in cargo_toml.parts or "tmp" in cargo_toml.parts:
                continue

            pkg_name = None
            pkg_version = None

            with open(cargo_toml, 'r') as f:
                for line in f:
                    if pkg_name is None:
                        match = re.match(r'^\s*name\s*=\s*"([^"]+)"', line)
                        if match:
                            pkg_name = match.group(1)

                    if pkg_version is None:
                        match = re.match(r'^\s*version\s*=\s*"([^"]+)"', line)
                        if match:
                            pkg_version = match.group(1)

                    if pkg_name and pkg_version:
                        break

            if pkg_name:
                normalized_name = pkg_name.replace("-", "_")
                packages[normalized_name] = (cargo_toml.parent, pkg_version or "0.0.0")

        return packages

    def process_file(cargo_toml: Path, packages: Dict[str, Tuple[Path, str]]) -> bool:
        """Process a Cargo.toml file. Returns True if modified."""
        with open(cargo_toml, 'r') as f:
            content = f.read()

        lines = content.split('\n')
        new_lines = []
        modified = False

        for line in lines:
            # Match dependency line: dep = { ... }
            match = re.match(r'^(\s*)([a-zA-Z0-9_-]+)\s*=\s*\{([^}]+)\}(.*)$', line)

            if not match:
                new_lines.append(line)
                continue

            indent = match.group(1)
            dep_name = match.group(2)
            attrs_str = match.group(3)
            rest = match.group(4).lstrip('}')  # Remove any extra closing braces

            normalized_dep = dep_name.replace("-", "_")

            # Only process workspace dependencies
            if normalized_dep not in packages:
                new_lines.append(line)
                continue

            dep_pkg_path, dep_version = packages[normalized_dep]

            # Parse existing attributes
            has_version = re.search(r'version\s*=\s*"([^"]+)"', attrs_str)
            has_path = re.search(r'path\s*=\s*"[^"]+"', attrs_str)

            # If no path attribute, skip (already version-only)
            if not has_path:
                new_lines.append(line)
                continue

            # Remove path, keep version and other attributes
            attrs_parts = []

            # Get version (existing or from target package)
            if has_version:
                version = has_version.group(1)
            else:
                version = dep_version
                modified = True

            attrs_parts.append(f'version = "{version}"')

            # Preserve other attributes (features, optional, etc.) but NOT path
            for attr_match in re.finditer(r'(features\s*=\s*\[[^\]]*\]|optional\s*=\s*\w+|default-features\s*=\s*\w+)', attrs_str):
                attrs_parts.append(attr_match.group(0))

            new_line = indent + dep_name + ' = { ' + ', '.join(attrs_parts) + ' }' + rest
            new_lines.append(new_line)
            modified = True

        if modified:
            with open(cargo_toml, 'w') as f:
                f.write('\n'.join(new_lines))

        return modified

    # Main execution
    packages = find_packages()
    updated_count = 0

    print("Converting to version-only dependencies...")
    print(f"Found {len(packages)} workspace packages")
    print()

    for pkg_name, (pkg_path, _) in sorted(packages.items()):
        cargo_toml = pkg_path / "Cargo.toml"

        if process_file(cargo_toml, packages):
            print(f"  ✓ {pkg_path.name}")
            updated_count += 1

    print()
    if updated_count == 0:
        print("All dependencies already version-only")
    else:
        print(f"Updated {updated_count} package(s)")

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
    @nohup sh -c 'cd ./packages/kodegen && cargo clean && cargo update && cargo install --path .' > ./tmp/mcp/kodegen.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-browser && cargo clean && cargo update && cargo install --path . --force && kodegen-browser --http 127.0.0.1:30438' > ./tmp/mcp/kodegen-browser.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-citescrape && cargo clean && cargo update && cargo install --path . --force && kodegen-citescrape --http 127.0.0.1:30439' > ./tmp/mcp/kodegen-citescrape.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-claude-agent && cargo clean && cargo update && cargo install --path . --force && kodegen-claude-agent --http 127.0.0.1:30440' > ./tmp/mcp/kodegen-claude-agent.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-config && cargo clean && cargo update && cargo install --path . --force && kodegen-config --http 127.0.0.1:30441' > ./tmp/mcp/kodegen-config.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-database && cargo clean && cargo update && cargo install --path . --force && kodegen-database --http 127.0.0.1:30442' > ./tmp/mcp/kodegen-database.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-filesystem && cargo clean && cargo update && cargo install --path . --force && kodegen-filesystem --http 127.0.0.1:30443' > ./tmp/mcp/kodegen-filesystem.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-git && cargo clean && cargo update && cargo install --path . --force && kodegen-git --http 127.0.0.1:30444' > ./tmp/mcp/kodegen-git.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-github && cargo clean && cargo update && cargo install --path . --force && kodegen-github --http 127.0.0.1:30445' > ./tmp/mcp/kodegen-github.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-introspection && cargo clean && cargo update && cargo install --path . --force && kodegen-introspection --http 127.0.0.1:30446' > ./tmp/mcp/kodegen-introspection.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-process && cargo clean && cargo update && cargo install --path . --force && kodegen-process --http 127.0.0.1:30447' > ./tmp/mcp/kodegen-process.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-prompt && cargo clean && cargo update && cargo install --path . --force && kodegen-prompt --http 127.0.0.1:30448' > ./tmp/mcp/kodegen-prompt.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-reasoner && cargo clean && cargo update && cargo install --path . --force && kodegen-reasoner --http 127.0.0.1:30449' > ./tmp/mcp/kodegen-reasoner.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-sequential-thinking && cargo clean && cargo update && cargo install --path . --force && kodegen-sequential-thinking --http 127.0.0.1:30450' > ./tmp/mcp/kodegen-sequential-thinking.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-tools-terminal && cargo clean && cargo update && cargo install --path . --force && kodegen-terminal --http 127.0.0.1:30451' > ./tmp/mcp/kodegen-terminal.log 2>&1 &
    @nohup sh -c 'cd ./packages/kodegen-candle-agent && cargo clean && cargo update && cargo install --path . --force && kodegen-candle-agent --http 127.0.0.1:30452' > ./tmp/mcp/kodegen-candle-agent.log 2>&1 &
    @tail -F ./tmp/mcp/*.log
