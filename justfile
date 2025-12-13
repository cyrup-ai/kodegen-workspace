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
        "kodegen-bash-shell"
        "kodegen-bundler-autoconfig"
        "kodegen-bundler-release"
        "kodegen-bundler-sign"
        "kodegen-candle-agent"
        "kodegen-claude-agent"
        "kodegen-mcp-client"
        "kodegen-mcp-schema"
        "kodegen-mcp-schema-macros"
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
        "kodegen-native-permissions"
        "kodegen-native-notify"
        "kodegend-protocol-ipc"
        "kodegend-client-ipc"
        "kodegend"
    )

    echo "Running cargo check and clippy on all projects..."
    echo "Failed projects will be saved to task/"
    echo ""

    failed_projects=()
    succeeded_projects=()

    for project in "${projects[@]}"; do
        if [ -d "packages/$project" ] && [ -f "packages/$project/Cargo.toml" ]; then
            echo "Checking $project..."
            output_file="task/${project}.txt"

            # Delete old file if it exists (ensures only failures generate files)
            rm -f "$output_file"

            # cargo update
            (cd "packages/$project" && cargo update > /dev/null 2>&1) || true

            # Capture cargo check output (treat warnings as errors)
            check_output=$(cd "packages/$project" && RUSTFLAGS="-D warnings" cargo check 2>&1)
            check_exit=$?

            # Capture cargo clippy output (treat warnings as errors)
            clippy_output=$(cd "packages/$project" && cargo clippy -- -D warnings 2>&1)
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

# Build candle-agent with platform-appropriate GPU features
build-candle:
    #!/usr/bin/env bash
    cd packages/kodegen-candle-agent
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Building with Metal GPU support for macOS..."
        cargo build --features metal
    elif [[ "$OSTYPE" == "linux-gnu"* ]] && [[ "$(uname -m)" == "x86_64" ]]; then
        echo "Building with CUDA GPU support for Linux x86_64..."
        cargo build --features cuda
    else
        echo "Building with CPU-only support..."
        cargo build
    fi

# Run candle examples with platform-appropriate GPU features
run-candle-example example_name:
    #!/usr/bin/env bash
    cd packages/kodegen-candle-agent
    if [[ "$OSTYPE" == "darwin"* ]]; then
        echo "Running example with Metal GPU support..."
        cargo run --example {{example_name}} --features metal
    elif [[ "$OSTYPE" == "linux-gnu"* ]] && [[ "$(uname -m)" == "x86_64" ]]; then
        echo "Running example with CUDA GPU support..."
        cargo run --example {{example_name}} --features cuda
    else
        echo "Running example with CPU-only support..."
        cargo run --example {{example_name}}
    fi

# Bump package version and update all workspace dependency references
bump package_name bump_type="patch":
    #!/usr/bin/env bash
    set -e

    package_name="{{ package_name }}"
    bump_type="{{ bump_type }}"

    # Validate bump type
    case "$bump_type" in
        major|minor|patch) ;;
        *)
            echo "Error: Invalid bump_type '$bump_type'"
            echo "Usage: just bump PACKAGE_NAME [major|minor|patch]"
            exit 1
            ;;
    esac

    # Validate package exists
    if [ ! -d "packages/$package_name" ]; then
        echo "Error: Package 'packages/$package_name' not found"
        echo ""
        echo "Available packages:"
        ls -1 packages/
        exit 1
    fi

    cargo_toml="packages/$package_name/Cargo.toml"

    if [ ! -f "$cargo_toml" ]; then
        echo "Error: $cargo_toml not found"
        exit 1
    fi

    # Extract current version from the package's Cargo.toml
    # Match the first 'version = "..."' line in the [package] section
    current_version=$(grep '^version = ' "$cargo_toml" | head -1 | sed 's/version = "\(.*\)"/\1/')

    if [ -z "$current_version" ]; then
        echo "Error: Could not find version in $cargo_toml"
        exit 1
    fi

    echo "Current version: $current_version"
    echo "Bump type: $bump_type"
    echo ""

    # Parse version into components
    IFS='.' read -r major minor patch <<< "$current_version"

    # Bump version based on type
    case "$bump_type" in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
    esac

    new_version="$major.$minor.$patch"
    new_dep_version="$major.$minor"

    echo "New version: $new_version"
    echo "New dependency version: $new_dep_version"
    echo ""

    # Update the package's own version in its Cargo.toml
    echo "Updating $cargo_toml..."
    perl -i -pe "s/^version = \".*\"/version = \"$new_version\"/ if \$. <= 10" "$cargo_toml"
    echo "  ✓ Updated package version to $new_version"
    echo ""

    echo "✓ Version bump complete!"
    echo ""
    echo "Summary:"
    echo "  Package: $package_name"
    echo "  Old version: $current_version"
    echo "  New version: $new_version"

# Convert workspace dependencies to local path dependencies (version + path)
dep-local:
    #!/usr/bin/env python3
    import os
    import re
    from pathlib import Path
    from typing import Dict, Tuple

    def find_packages() -> Dict[str, Tuple[Path, str]]:
        """Find all top-level packages and their versions. Skips nested packages and test fixtures."""
        packages = {}
        packages_dir = Path("packages")

        if not packages_dir.exists():
            return packages

        # Only process top-level packages (packages/*/Cargo.toml)
        # This skips nested packages like test fixtures
        for pkg_dir in sorted(packages_dir.iterdir()):
            if not pkg_dir.is_dir():
                continue

            # Skip hidden directories and special directories
            if pkg_dir.name.startswith('.') or pkg_dir.name in ['target', 'tmp']:
                continue

            cargo_toml = pkg_dir / "Cargo.toml"
            if not cargo_toml.exists():
                continue

            pkg_name = None
            pkg_version = None

            try:
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
                    packages[normalized_name] = (pkg_dir, pkg_version or "0.0.0")
            except Exception as e:
                print(f"  ⚠ Warning: Failed to read {cargo_toml}: {e}")

        return packages

    def calculate_relative_path(from_path: Path, to_path: Path) -> str:
        """Calculate relative path between packages."""
        rel = os.path.relpath(to_path, from_path)
        return rel.replace(os.sep, "/")

    def to_major_minor(version: str) -> str:
        """Convert version string to major.minor format (e.g., '0.2.0' -> '0.2')."""
        parts = version.split('.')
        if len(parts) >= 2:
            return f"{parts[0]}.{parts[1]}"
        return version

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

            # Get version (existing or from target package) and convert to major.minor
            if has_version:
                version = to_major_minor(has_version.group(1))
            else:
                version = to_major_minor(dep_version)
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

    if not packages:
        print("No packages found in packages/ directory")
        exit(0)

    updated_count = 0

    print("Converting to local path dependencies (version + path)...")
    print(f"Found {len(packages)} workspace packages")
    print()

    for pkg_name, (pkg_path, _) in sorted(packages.items()):
        cargo_toml = pkg_path / "Cargo.toml"

        try:
            if process_file(cargo_toml, packages, pkg_path):
                print(f"  ✓ {pkg_path.name}")
                updated_count += 1
        except Exception as e:
            print(f"  ✗ Failed to process {pkg_path.name}: {e}")

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
        """Find all top-level packages and their versions. Skips nested packages and test fixtures."""
        packages = {}
        packages_dir = Path("packages")

        if not packages_dir.exists():
            return packages

        # Only process top-level packages (packages/*/Cargo.toml)
        # This skips nested packages like test fixtures
        for pkg_dir in sorted(packages_dir.iterdir()):
            if not pkg_dir.is_dir():
                continue

            # Skip hidden directories and special directories
            if pkg_dir.name.startswith('.') or pkg_dir.name in ['target', 'tmp']:
                continue

            cargo_toml = pkg_dir / "Cargo.toml"
            if not cargo_toml.exists():
                continue

            pkg_name = None
            pkg_version = None

            try:
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
                    packages[normalized_name] = (pkg_dir, pkg_version or "0.0.0")
            except Exception as e:
                print(f"  ⚠ Warning: Failed to read {cargo_toml}: {e}")

        return packages

    def to_major_minor(version: str) -> str:
        """Convert version string to major.minor format (e.g., '0.2.0' -> '0.2')."""
        parts = version.split('.')
        if len(parts) >= 2:
            return f"{parts[0]}.{parts[1]}"
        return version

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

            # Get version (existing or from target package) and convert to major.minor
            if has_version:
                version = to_major_minor(has_version.group(1))
            else:
                version = to_major_minor(dep_version)
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

    if not packages:
        print("No packages found in packages/ directory")
        exit(0)

    updated_count = 0

    print("Converting to version-only dependencies...")
    print(f"Found {len(packages)} workspace packages")
    print()

    for pkg_name, (pkg_path, _) in sorted(packages.items()):
        cargo_toml = pkg_path / "Cargo.toml"

        try:
            if process_file(cargo_toml, packages):
                print(f"  ✓ {pkg_path.name}")
                updated_count += 1
        except Exception as e:
            print(f"  ✗ Failed to process {pkg_path.name}: {e}")

    print()
    if updated_count == 0:
        print("All dependencies already version-only")
    else:
        print(f"Updated {updated_count} package(s)")

# Publish changed packages to crates.io with version bumping and git tagging
publish bump_type:
    #!/usr/bin/env bash
    set -e

    bump_type="{{ bump_type }}"

    # Validate bump type
    case "$bump_type" in
        major|minor|patch|patch-all) ;;
        *)
            echo "Error: Invalid bump_type '$bump_type'"
            echo "Usage: just publish [major|minor|patch|patch-all]"
            exit 1
            ;;
    esac

    echo "📦 KODEGEN.ᴀɪ Package Publisher"
    echo "================================"
    echo ""

    # Phase 1: Validation
    echo "Validating workspace..."
    just dep-local
    echo "  ✓ Local dependencies configured"

    just check
    if [ $? -ne 0 ]; then
        echo "❌ Validation failed - aborting publish"
        exit 1
    fi
    echo "  ✓ All packages pass checks"

    just dep-published
    echo "  ✓ Switched to published dependencies"
    echo ""

    # Phase 2: Detect changes
    echo "Detecting changed packages..."

    changed_packages=()

    if [ "${bump_type}" = "major" ] || [ "${bump_type}" = "minor" ] || [ "${bump_type}" = "patch-all" ]; then
        # For major/minor/patch-all: publish ALL packages
        echo "  Bump type is ${bump_type} - publishing all packages"
        for pkg_dir in packages/*; do
            if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
                pkg_name=$(basename "$pkg_dir")
                changed_packages+=("$pkg_name")
            fi
        done
    else
        # For patch: only publish packages with changes
        echo "  Bump type is ${bump_type} - detecting changed packages"
        for pkg_dir in packages/*; do
            if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
                pkg_name=$(basename "$pkg_dir")

                # Check for changes using git status --porcelain
                if [ -n "$(cd "$pkg_dir" && git status --porcelain 2>/dev/null)" ]; then
                    changed_packages+=("$pkg_name")
                fi
            fi
        done
    fi

    if [ ${#changed_packages[@]} -eq 0 ]; then
        echo "No packages with changes found."
        echo "Nothing to publish."
        exit 0
    fi

    echo "Found ${#changed_packages[@]} package(s) to publish:"
    for pkg in "${changed_packages[@]}"; do
        echo "  - $pkg"
    done
    echo ""

    # Phase 3: Publish in dependency order
    echo "Publishing packages in dependency order..."
    echo ""

    # Hardcoded dependency order (topologically sorted)
    PUBLISH_ORDER=(
        # Level 0: Foundation (no internal dependencies)
        "kodegen-bash-shell"
        "kodegen-config"
        "kodegen-mcp-schema-macros"
        "kodegen-mcp-schema"
        "kodegen-simd"
        "kgls"
        "cylo"
        "kodegen-bundler-autoconfig"
        "kodegen-bundler-sign"
        "kodegen-native-permissions"
        "kodegend-protocol-ipc"

        # Level 1: Infrastructure layer (depends on Level 0)
        "kodegen-native-notify"
        "kodegen-mcp-client"
        "kodegen-config-manager"
        "kodegen-server-http"
        "kodegend-client-ipc"

        # Level 2: Individual tool packages
        "kodegen-tools-process"
        "kodegen-tools-prompt"
        "kodegen-tools-config"
        "kodegen-tools-introspection"
        "kodegen-tools-sequential-thinking"
        "kodegen-tools-database"
        "kodegen-tools-filesystem"
        "kodegen-tools-git"
        "kodegen-tools-github"
        "kodegen-tools-citescrape"
        "kodegen-tools-terminal"

        # Level 3: Tools with tool dependencies
        "kodegen-candle-agent"
        "kodegen-tools-browser"
        "kodegen-tools-reasoner"
        "kodegen-claude-agent"

        # Level 4: Applications
        "kodegen"
        "kodegen-bundler-bundle"
        "kodegen-bundler-release"

        # Level 5: Daemon (depends on everything)
        "kodegend"
    )

    published_packages=()
    count=0
    total=${#changed_packages[@]}

    for package in "${PUBLISH_ORDER[@]}"; do
        # Check if this package has changes
        if [[ " ${changed_packages[@]} " =~ " ${package} " ]]; then
            count=$((count + 1))
            echo "[${count}/${total}] Publishing ${package}..."

            # Navigate to package
            cd "packages/${package}"

            # Get current version
            old_version=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')

            # Determine actual bump type for versioning (patch-all -> patch)
            actual_bump_type="${bump_type}"
            if [ "${bump_type}" = "patch-all" ]; then
                actual_bump_type="patch"
            fi

            # Bump version
            cd ../..
            just bump "${package}" "${actual_bump_type}"

            # Get new version
            new_version=$(grep '^version = ' "packages/${package}/Cargo.toml" | head -1 | sed 's/version = "\(.*\)"/\1/')
            echo "  Bumped version: ${old_version} → ${new_version}"

            # Update dependencies
            cd "packages/${package}"
            echo "  Running cargo update..."
            cargo update
            echo "  ✓ Dependencies updated"

            # Commit and tag inside submodule
            echo "  Committing and tagging in submodule..."
            git add .
            git commit -m "v${new_version}"
            git tag -a "v${new_version}" -m "Release v${new_version}"
            git push origin main
            git push --tags
            echo "  ✓ Committed and tagged in submodule"

            # Publish to crates.io
            echo "  Publishing to crates.io..."
            cargo publish

            if [ $? -ne 0 ]; then
                echo "  ❌ Failed to publish ${package}"
                cd ../..
                exit 1
            fi

            cd ../..

            # For major/minor: update dependent package references
            if [ "${actual_bump_type}" = "major" ] || [ "${actual_bump_type}" = "minor" ]; then
                echo "  Updating dependent package references..."

                # Extract major.minor from new_version (e.g., "0.4.0" -> "0.4")
                dep_version=$(echo "${new_version}" | cut -d. -f1-2)

                # Convert package name to dependency format (hyphen to underscore)
                dep_package_name=$(echo "${package}" | tr '-' '_')

                # Update all Cargo.toml files that depend on this package
                updated_count=0
                for other_cargo in packages/*/Cargo.toml; do
                    other_pkg=$(basename $(dirname "$other_cargo"))

                    # Skip the package we just published
                    if [ "$other_pkg" = "${package}" ]; then
                        continue
                    fi

                    # Check if this package depends on the one we just published
                    if grep -q "^${dep_package_name} = " "$other_cargo" || \
                       grep -q "^[[:space:]]*${dep_package_name} = " "$other_cargo"; then

                        # Update the version reference
                        perl -i -pe "s/^(\\s*)${dep_package_name} = \\{ version = \"[^\"]*\"/\$1${dep_package_name} = { version = \"${dep_version}\"/" "$other_cargo"

                        echo "    ✓ Updated ${other_pkg} to use ${dep_package_name} v${dep_version}"
                        updated_count=$((updated_count + 1))
                    fi
                done

                if [ $updated_count -gt 0 ]; then
                    echo "  Updated ${updated_count} dependent package(s)"
                fi
            fi

            echo "  ✓ Published ${package}@${new_version}"
            echo ""

            published_packages+=("${package}@${new_version}")
        fi
    done

    # Phase 4: Sync workspace after all publishes
    if [ ${#published_packages[@]} -gt 0 ]; then
        echo "Syncing workspace after publish..."

        # Always commit - submodule references have changed
        git add .
        git commit -m "chore: sync workspace after package releases"
        git push origin main
        echo "  ✓ Workspace synced"

        echo ""
        echo "✅ Published ${#published_packages[@]} package(s) successfully!"
        echo ""
        echo "Packages published:"
        for pkg in "${published_packages[@]}"; do
            echo "  - ${pkg}"
        done
    fi

# List all projects
mcp:
    @echo 'Stopping old MCP processes ...'
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
    @pkill -f kodegend || true
    @rm -rf ./tmp/mcp
    @sleep 2
    @mkdir -p ./tmp/mcp
    @echo 'Starting new MCP processes ...'
    @nohup sh -c 'cargo install kodegen' > ./tmp/mcp/kodegen.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_browser && kodegen-browser --http 127.0.0.1:30438' > ./tmp/mcp/kodegen-browser.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_citescrape && kodegen-citescrape --http 127.0.0.1:30439' > ./tmp/mcp/kodegen-citescrape.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_claude_agent && kodegen-claude-agent --http 127.0.0.1:30440' > ./tmp/mcp/kodegen-claude-agent.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_config && kodegen-config --http 127.0.0.1:30441' > ./tmp/mcp/kodegen-config.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_database && kodegen-database --http 127.0.0.1:30442' > ./tmp/mcp/kodegen-database.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_filesystem && kodegen-filesystem --http 127.0.0.1:30443' > ./tmp/mcp/kodegen-filesystem.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_git && kodegen-git --http 127.0.0.1:30444' > ./tmp/mcp/kodegen-git.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_github && kodegen-github --http 127.0.0.1:30445' > ./tmp/mcp/kodegen-github.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_introspection && kodegen-introspection --http 127.0.0.1:30446' > ./tmp/mcp/kodegen-introspection.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_process && kodegen-process --http 127.0.0.1:30447' > ./tmp/mcp/kodegen-process.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_prompt && kodegen-prompt --http 127.0.0.1:30448' > ./tmp/mcp/kodegen-prompt.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_reasoner && kodegen-reasoner --http 127.0.0.1:30449' > ./tmp/mcp/kodegen-reasoner.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_sequential_thinking && kodegen-sequential-thinking --http 127.0.0.1:30450' > ./tmp/mcp/kodegen-sequential-thinking.log 2>&1 &
    @nohup sh -c 'cargo install kodegen_tools_terminal && kodegen-terminal --http 127.0.0.1:30451' > ./tmp/mcp/kodegen-terminal.log 2>&1 &
    @nohup sh -c ' \
        if [[ "$$OSTYPE" == "darwin"* ]]; then \
            cargo install kodegen_candle_agent --features metal; \
        elif [[ "$$OSTYPE" == "linux-gnu"* ]] && [[ "$$(uname -m)" == "x86_64" ]]; then \
            cargo install kodegen_candle_agent --features cuda; \
        else \
            cargo install kodegen_candle_agent; \
        fi && kodegen-candle-agent --http 127.0.0.1:30452' > ./tmp/mcp/kodegen-candle-agent.log 2>&1 &
    @tail -F ./tmp/mcp/*.log

# Yank all published versions matching a minor version (e.g., just yank 0.1)
yank minor_version:
    #!/usr/bin/env bash

    minor_version="{{ minor_version }}"

    # Validate minor version format
    if ! [[ "$minor_version" =~ ^[0-9]+\.[0-9]+$ ]]; then
        echo "Error: Invalid minor version format. Use X.Y (e.g., '0.1', '1.2')"
        exit 1
    fi

    echo "========================================="
    echo "Yanking all versions matching $minor_version.*"
    echo "========================================="
    echo ""

    # Check for jq (required for JSON parsing)
    if ! command -v jq &> /dev/null; then
        echo "Error: jq is required but not installed."
        echo "Install with: brew install jq (macOS) or apt-get install jq (Linux)"
        exit 1
    fi

    # Discover packages
    echo "Discovering packages..."
    packages=()
    for pkg_dir in packages/*; do
        if [ -d "$pkg_dir" ] && [ -f "$pkg_dir/Cargo.toml" ]; then
            pkg_name=$(grep '^name = ' "$pkg_dir/Cargo.toml" | head -1 | sed 's/name = "\(.*\)"/\1/')
            packages+=("$pkg_name")
        fi
    done
    echo "Found ${#packages[@]} packages"
    echo ""

    # Track statistics
    total_yanked=0
    total_skipped=0
    failed_yanks=()

    # Process each package
    for crate_name in "${packages[@]}"; do
        echo "Processing: $crate_name"

        # Query crates.io for all versions
        echo "  Querying crates.io..."
        api_response=$(curl -s "https://crates.io/api/v1/crates/$crate_name")

        if [ $? -ne 0 ] || [ -z "$api_response" ]; then
            echo "  ⚠ Failed to query crates.io"
            total_skipped=$((total_skipped + 1))
            continue
        fi

        # Check if crate exists
        if echo "$api_response" | jq -e '.errors' &> /dev/null; then
            echo "  ⚠ Crate not published (not found on crates.io)"
            total_skipped=$((total_skipped + 1))
            continue
        fi

        # Extract versions matching minor version that are NOT already yanked
        versions_to_yank=($(echo "$api_response" | jq -r '.versions[] | select(.yanked == false) | .num' | grep "^${minor_version}\." || true))

        if [ ${#versions_to_yank[@]} -eq 0 ]; then
            echo "  No versions matching $minor_version.*"
            total_skipped=$((total_skipped + 1))
            continue
        fi

        echo "  Found ${#versions_to_yank[@]} version(s) to yank: ${versions_to_yank[*]}"

        # Yank each version
        for i in "${!versions_to_yank[@]}"; do
            version="${versions_to_yank[$i]}"
            echo "  Yanking $crate_name@$version..."

            if cargo yank --vers "$version" "$crate_name" 2>&1; then
                echo "    ✓ Successfully yanked"
                total_yanked=$((total_yanked + 1))
            else
                echo "    ✗ Failed to yank"
                failed_yanks+=("$crate_name@$version")
            fi

            # Sleep 15 seconds between yanks (except after the last one)
            if [ $i -lt $((${#versions_to_yank[@]} - 1)) ]; then
                echo "    Waiting 15 seconds before next yank..."
                sleep 15
            fi
        done

        echo ""
    done

    # Summary
    echo "========================================="
    echo "Summary"
    echo "========================================="
    echo "Total packages processed: ${#packages[@]}"
    echo "Total versions yanked: $total_yanked"
    echo "Packages skipped: $total_skipped"

    if [ ${#failed_yanks[@]} -gt 0 ]; then
        echo ""
        echo "Failed to yank (${#failed_yanks[@]}):"
        printf '  %s\n' "${failed_yanks[@]}"
        exit 1
    else
        echo ""
        echo "All yanks completed successfully!"
    fi
