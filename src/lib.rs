pub mod resource;
pub mod lsf;
pub mod lsx;
pub mod compression;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::fs;
    use std::process::Command;

    /// Extract readable strings from an LSF file using the `strings` command
    fn extract_strings_from_lsf(file_path: &Path) -> Vec<String> {
        let output = Command::new("strings")
            .arg(file_path)
            .output()
            .expect("Failed to run strings command");
        
        if !output.status.success() {
            panic!("strings command failed");
        }
        
        let strings_output = String::from_utf8_lossy(&output.stdout);
        strings_output
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && s.len() >= 3 && s.chars().all(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace()))
            .filter(|s| s != "LSOF") // Filter out the magic header
            .collect()
    }

    /// Get all LSF files from the assets directory
    fn get_lsf_files() -> Vec<PathBuf> {
        let assets_dir = Path::new("assets");
        if !assets_dir.exists() {
            panic!("Assets directory not found");
        }
        
        fs::read_dir(assets_dir)
            .expect("Failed to read assets directory")
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "lsf" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect()
    }

    #[test]
    fn test_lsf_reading_all_files() {
        let lsf_files = get_lsf_files();
        assert!(!lsf_files.is_empty(), "No LSF files found in assets directory");
        
        for test_file in lsf_files {
            println!("\n=== Testing file: {} ===", test_file.display());
            
            match lsf::read_lsf(&test_file) {
                Ok(resource) => {
                    println!("Successfully read LSF file!");
                    println!("Metadata: {:?}", resource.metadata);
                    println!("Regions count: {}", resource.regions.len());
                    
                    for (region_name, region) in &resource.regions {
                        println!("Region '{}' has {} nodes", region_name, region.nodes.len());
                        
                        for (i, node) in region.nodes.iter().enumerate().take(3) {
                            println!("  Node {}: id='{}', {} attributes", i, node.id, node.attributes.len());
                            for (attr_name, attr) in node.attributes.iter().take(3) {
                                println!("    {}: {:?}", attr_name, attr.attribute_type);
                            }
                        }
                    }
                },
                Err(e) => {
                    panic!("Failed to read LSF file {}: {}", test_file.display(), e);
                }
            }
        }
    }

    #[test]
    fn test_string_preservation_all_files() {
        let lsf_files = get_lsf_files();
        assert!(!lsf_files.is_empty(), "No LSF files found in assets directory");
        
        for test_file in lsf_files {
            println!("\n=== Testing string preservation for: {} ===", test_file.display());
            
            // Extract strings using the `strings` command
            let expected_strings = extract_strings_from_lsf(&test_file);
            println!("Found {} strings in file: {:?}", expected_strings.len(), &expected_strings[..expected_strings.len().min(10)]);
            
            if expected_strings.is_empty() {
                println!("No meaningful strings found, skipping test for this file");
                continue;
            }
            
            // Read LSF file
            let resource = lsf::read_lsf(&test_file).expect("Failed to read LSF");
            
            // Convert to LSX
            let output_filename = format!("test_strings_{}.lsx", test_file.file_stem().unwrap().to_string_lossy());
            let output_file = Path::new(&output_filename);
            lsx::write_lsx(&resource, output_file).expect("Failed to write LSX");
            
            // Read the generated XML as a string
            let xml_content = fs::read_to_string(output_file).expect("Failed to read LSX file");
            
            println!("Generated XML content (first 500 chars):\n{}", &xml_content[..xml_content.len().min(500)]);
            
            // Check that at least some of the extracted strings appear in the output
            let mut found_strings = 0;
            let strings_to_check = expected_strings.iter().take(5); // Check first 5 strings
            
            for expected_string in strings_to_check {
                let found = xml_content.contains(expected_string) || 
                    resource.regions.keys().any(|k| k.contains(expected_string)) ||
                    resource.regions.values().any(|region| {
                        region.nodes.iter().any(|node| {
                            node.name.as_ref().map_or(false, |name| name.contains(expected_string)) ||
                            node.attributes.keys().any(|attr| attr.contains(expected_string)) ||
                            node.attributes.values().any(|attr| {
                                match &attr.value {
                                    resource::AttributeValue::String(s) |
                                    resource::AttributeValue::Path(s) |
                                    resource::AttributeValue::FixedString(s) |
                                    resource::AttributeValue::LSString(s) => s.contains(expected_string),
                                    _ => false
                                }
                            })
                        })
                    });
                
                if found {
                    found_strings += 1;
                    println!("✓ Found string: '{}'", expected_string);
                } else {
                    println!("✗ Missing string: '{}'", expected_string);
                }
            }
            
            // We expect at least one string to be preserved
            assert!(
                found_strings > 0,
                "No expected strings found in LSX output for file {}. Expected at least one of: {:?}", 
                test_file.display(),
                expected_strings.iter().take(5).collect::<Vec<_>>()
            );
            
            // Clean up
            fs::remove_file(output_file).ok();
            
            println!("String preservation test passed for {}! Found {}/{} strings", 
                     test_file.display(), found_strings, expected_strings.len().min(5));
        }
    }

    #[test]
    fn test_round_trip_conversion_all_files() {
        let lsf_files = get_lsf_files();
        assert!(!lsf_files.is_empty(), "No LSF files found in assets directory");
        
        for test_file in lsf_files {
            println!("\n=== Testing round-trip conversion for: {} ===", test_file.display());
            
            // Step 1: Read original LSF
            let original_resource = lsf::read_lsf(&test_file).expect("Failed to read original LSF");
            
            // Step 2: Convert to LSX
            let lsx_filename = format!("test_roundtrip_{}.lsx", test_file.file_stem().unwrap().to_string_lossy());
            let lsx_file = Path::new(&lsx_filename);
            lsx::write_lsx(&original_resource, lsx_file).expect("Failed to write LSX");
            
            // Step 3: Read LSX back
            let lsx_resource = lsx::read_lsx(lsx_file).expect("Failed to read LSX");
            
            // Step 4: Convert back to LSF
            let roundtrip_lsf_filename = format!("test_roundtrip_{}.lsf", test_file.file_stem().unwrap().to_string_lossy());
            let roundtrip_lsf_file = Path::new(&roundtrip_lsf_filename);
            lsf::write_lsf(&lsx_resource, roundtrip_lsf_file).expect("Failed to write LSF");
            
            // Step 5: Compare original and round-trip LSF files
            let original_bytes = fs::read(&test_file).expect("Failed to read original LSF");
            let roundtrip_bytes = fs::read(roundtrip_lsf_file).expect("Failed to read round-trip LSF");
            
            // Check that the files are reasonably similar in size 
            let size_ratio = roundtrip_bytes.len() as f64 / original_bytes.len() as f64;
            assert!(
                size_ratio > 0.3 && size_ratio < 3.0,
                "Round-trip file size differs significantly for {}: original {} bytes, round-trip {} bytes (ratio: {:.2})",
                test_file.display(),
                original_bytes.len(),
                roundtrip_bytes.len(),
                size_ratio
            );
            
            // Verify that we can read the round-trip file successfully
            let roundtrip_resource = lsf::read_lsf(roundtrip_lsf_file).expect("Failed to read round-trip LSF");
            
            // Check that basic structure is preserved
            assert!(!roundtrip_resource.regions.is_empty(), "Round-trip file should have at least one region for {}", test_file.display());
            
            // Clean up
            fs::remove_file(lsx_file).ok();
            fs::remove_file(roundtrip_lsf_file).ok();
            
            println!("Round-trip test passed for {}! Size ratio: {:.2}", test_file.display(), size_ratio);
        }
    }
    
    #[test]
    fn test_diagnose_profile8_parsing() {
        let test_file = Path::new("assets/profile8.lsf");
        if !test_file.exists() {
            return;
        }

        println!("=== DIAGNOSTIC ANALYSIS FOR profile8.lsf ===");
        
        // Extract strings using command
        let expected_strings = extract_strings_from_lsf(&test_file);
        println!("Strings command found {} strings", expected_strings.len());
        
        // Try to read with detailed diagnostics
        match lsf::read_lsf(&test_file) {
            Ok(resource) => {
                println!("✓ Successfully parsed LSF file");
                println!("  Regions found: {}", resource.regions.len());
                
                let mut total_nodes = 0;
                let mut nodes_with_attrs = 0;
                let mut total_attrs = 0;
                
                for (region_name, region) in &resource.regions {
                    total_nodes += region.nodes.len();
                    for node in &region.nodes {
                        if !node.attributes.is_empty() {
                            nodes_with_attrs += 1;
                            total_attrs += node.attributes.len();
                        }
                    }
                    println!("  Region '{}': {} nodes", region_name, region.nodes.len());
                }
                
                println!("  Total nodes: {}", total_nodes);
                println!("  Nodes with attributes: {}", nodes_with_attrs);
                println!("  Total attributes: {}", total_attrs);
                
                // Compare with what we expected from file size
                let file_size = std::fs::metadata(&test_file).unwrap().len();
                println!("  Original file size: {} bytes", file_size);
                println!("  Data utilization: {:.1}%", (total_attrs * 50) as f64 / file_size as f64 * 100.0);
                
                // Now test round-trip to see data loss
                let lsx_file = Path::new("diagnostic_profile8.lsx");
                lsx::write_lsx(&resource, lsx_file).expect("Failed to write LSX");
                
                let lsx_resource = lsx::read_lsx(lsx_file).expect("Failed to read LSX");
                let roundtrip_lsf_file = Path::new("diagnostic_profile8_roundtrip.lsf");
                lsf::write_lsf(&lsx_resource, roundtrip_lsf_file).expect("Failed to write LSF");
                
                let original_size = std::fs::metadata(&test_file).unwrap().len();
                let roundtrip_size = std::fs::metadata(roundtrip_lsf_file).unwrap().len();
                let ratio = roundtrip_size as f64 / original_size as f64;
                
                println!("  Round-trip size ratio: {:.3}", ratio);
                println!("  Data loss: {:.1}%", (1.0 - ratio) * 100.0);
                
                // Clean up
                std::fs::remove_file(lsx_file).ok();
                std::fs::remove_file(roundtrip_lsf_file).ok();
                
            },
            Err(e) => {
                println!("✗ Failed to parse LSF file: {}", e);
            }
        }
    }

    #[test]
    fn test_lsf_to_lsx_conversion_all_files() {
        let lsf_files = get_lsf_files();
        assert!(!lsf_files.is_empty(), "No LSF files found in assets directory");
        
        for test_file in lsf_files {
            println!("\n=== Testing LSF to LSX conversion for: {} ===", test_file.display());
            
            let resource = lsf::read_lsf(&test_file).expect("Failed to read LSF");
            
            let output_filename = format!("test_output_{}.lsx", test_file.file_stem().unwrap().to_string_lossy());
            let output_file = Path::new(&output_filename);
            lsx::write_lsx(&resource, output_file).expect("Failed to write LSX");
            
            // Verify the XML file was created and can be read back
            assert!(output_file.exists());
            
            let resource2 = lsx::read_lsx(output_file).expect("Failed to read back LSX");
            
            // Basic verification
            assert_eq!(resource.regions.len(), resource2.regions.len());
            
            // Clean up
            fs::remove_file(output_file).ok();
            
            println!("LSF to LSX conversion test passed for {}!", test_file.display());
        }
    }
}