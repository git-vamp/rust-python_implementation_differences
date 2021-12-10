from os import mkdir

def main():

    structure = {
        "store": 1,
        "aisle":1,
        "products":5,
    }

    base_path = "base_dir/"
    mkdir(base_path)
    for i in structure.items():
        for q in range(1, i[1]+ 1):

            formated_folder_structure = f"{base_path}{i[0]}_{q}"
            if q == i[1]:
                mkdir(formated_folder_structure)        
                base_path +=  f"{i[0]}_{q}/"
            else:
                mkdir(formated_folder_structure)
            print(formated_folder_structure)

if __name__ == "__main__":
    main()
    