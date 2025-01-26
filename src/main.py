import os
import argparse

def divide_photos(path: str):
    os.chdir(path)

    # Create directories if they don't exist
    for d in ['jpg', 'raf', 'jpg/selected', 'raf/selected']:
        try:
            os.makedirs(d, exist_ok=True)
        except Exception as e:
            print(f"Failed to create directory {d}: {e}")
            return

    # List all entries in the current directory
    for entry in os.listdir('.'):
        if os.path.isfile(entry):
            filename = os.path.basename(entry)
            extension = os.path.splitext(filename)[1].split('.')[1]
            
            if extension.lower() in ['jpg', 'raf']:
                new_path = f'{extension.lower()}/{filename}'
                print(f'Moving {filename} to {new_path}')
                os.rename(entry, new_path)
            else:
                print(f'Omitting {filename}')

    print('Done moving photos')

def select_raf_from_jpg(path: str):
    os.chdir(path)

    raf_selected = 'raf/selected'
    
    try:
        os.makedirs(raf_selected, exist_ok=True)
    except Exception as e:
        print(f"Failed to create directory {raf_selected}: {e}")
        return
    
    # Iterate over files in jpg/selected/
    if os.path.isdir('jpg/selected'):
        for entry in os.listdir('jpg/selected'):
            print(f'Selecting {entry}')
            RAF_filename = entry.replace('.JPG', '.RAF')
            source_raf_path = f'raf/{RAF_filename}'
            dest_raf_path = f'raf/selected/{RAF_filename}'
            
            if os.path.exists(dest_raf_path):
                print(f'Omitting {entry}')
                continue

            os.rename(source_raf_path, dest_raf_path)
        print('Done copying RAW files')

def main():
    parser = argparse.ArgumentParser(description="Manage JPG and RAF photo files.")
    parser.add_argument(
        '--divide', metavar='FOLDER', type=str, help="Organize JPG and RAF photos into folders."
    )
    parser.add_argument(
        '--select', metavar='FOLDER', type=str, help="Move RAF files corresponding to JPGs in 'jpg/selected'."
    )

    args = parser.parse_args()

    if args.divide:
        divide_photos(args.divide)
    elif args.select:
        select_raf_from_jpg(args.select)
    else:
        print("Error: You must specify either --divide or --select with a folder path.")

if __name__ == '__main__':
    main()