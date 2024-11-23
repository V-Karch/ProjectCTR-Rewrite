import os

def main(args):
    # Allocate and initialize user settings
    set = {"common": {}, "ncch": {}, "workingFile": {}}
    if not set:
        print("[!] Not enough memory")
        return -1

    initialize_user_settings(set) # type: ignore
    initialize_random_generator() # type: ignore

    try:
        # Parse arguments
        result = parse_arguments(args, set) # type: ignore
        if result < 0:
            raise Exception("Argument parsing failed")

        # Get RSF settings
        result = get_rsf_settings(set) # type: ignore
        if result < 0:
            raise Exception("Failed to get RSF settings")

        # NCCH handling
        if not set["ncch"].get("buildNcch0"):
            if set["common"].get("workingFileType") == "infile_ncch":
                content_path = set["common"].get("contentPath", [None])[0]
                if not os.path.exists(content_path):
                    print(f"[MAKEROM ERROR] Failed to open Content 0: {content_path}")
                    raise Exception("File not found")
                
                file_size = get_file_size(content_path) # type: ignore
                with open(content_path, "rb") as ncch_file:
                    hdr = read_ncch_header(ncch_file) # type: ignore
                    calc_size = calculate_ncch_size(hdr) # type: ignore

                    if calc_size != file_size:
                        print("[MAKEROM ERROR] Content 0 is corrupt")
                        raise Exception("File corruption detected")
                    
                    set["common"]["workingFile"] = {
                        "size": file_size,
                        "buffer": ncch_file.read()
                    }
            else:
                working_file_path = set["common"].get("workingFilePath")
                if not os.path.exists(working_file_path):
                    print(f"[MAKEROM ERROR] Failed to open: {working_file_path}")
                    raise Exception("File not found")
                
                file_size = get_file_size(working_file_path) # type: ignore
                aligned_size = align_to_boundary(file_size, 16) # type: ignore
                with open(working_file_path, "rb") as fp:
                    set["common"]["workingFile"] = {
                        "size": aligned_size,
                        "buffer": fp.read()
                    }
        else:
            result = build_ncch(set) # type: ignore
            if result < 0:
                print(f"[RESULT] Failed to build NCCH (ret = {result})")
                raise Exception("NCCH build failed")

        # Handle output formats
        if set["common"].get("outFormat") == "CCI":
            result = build_cci(set) # type: ignore
            if result < 0:
                raise Exception("Failed to build CCI")
        elif set["common"].get("outFormat") == "CIA":
            result = build_cia(set) # type: ignore
            if result < 0:
                raise Exception("Failed to build CIA")
        elif set["common"].get("outFormat") in ["CXI", "CFA"]:
            output_file_name = set["common"].get("outFileName")
            try:
                with open(output_file_name, "wb") as ncch_out:
                    ncch_out.write(set["common"]["workingFile"]["buffer"])
            except Exception:
                print(f"[MAKEROM ERROR] Failed to create '{output_file_name}'")
                raise Exception("Failed to create output file")
    except Exception as e:
        print(e)
        return -1
    finally:
        # Free resources
        free_user_settings(set) # type: ignore

    return result
