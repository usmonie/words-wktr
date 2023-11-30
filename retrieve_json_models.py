import json
from collections import defaultdict

def merge_structures(base, new, key_counter, path=""):
    """ Объединяет две структуры, добавляя уникальные поля из 'new' в 'base'. """
    if isinstance(base, dict) and isinstance(new, dict):
        for key in new:
            new_path = f"{path}.{key}" if path else key
            if key in base:
                base[key] = merge_structures(base[key], new[key], key_counter, new_path)
            else:
                base[key] = new[key]
            key_counter[new_path] += 1
    elif isinstance(base, list) and isinstance(new, list) and new:
        base_structure = {}
        for item in new:
            base_structure = merge_structures(base_structure, item, key_counter, path)
        return [base_structure]
    else:
        return new if new is not None else base
    return base

def mark_optional_fields(structure, key_counter, total_objects):
    """ Помечает поля как опциональные, если они не встречаются в каждом объекте. """
    if isinstance(structure, dict):
        for key in structure:
            full_key = f"{key}" if not structure.get('__path__') else f"{structure['__path__']}.{key}"
            if key_counter[full_key] < total_objects:
                structure[key] = {"optional": True, "example": structure[key]}
            if isinstance(structure[key], dict):
                structure[key]['__path__'] = full_key
                mark_optional_fields(structure[key], key_counter, total_objects)
            elif isinstance(structure[key], list) and structure[key]:
                mark_optional_fields(structure[key][0], key_counter, total_objects)
    return structure

def process_file(filename):
    structure = {}
    key_counter = defaultdict(int)
    total_objects = 0
    with open(filename, 'r', encoding='utf-8') as file:
        for line in file:
            try:
                json_obj = json.loads(line)
                structure = merge_structures(structure, json_obj, key_counter)
                total_objects += 1
            except json.JSONDecodeError:
                pass  # Пропускаем недопустимые JSON строки
            return mark_optional_fields(structure, key_counter, total_objects)

def write_to_file(data, filename):
    """ Записывает данные в файл в формате JSON. """
    with open(filename, 'w', encoding='utf-8') as file:
        json.dump(data, file, indent=4, ensure_ascii=False)

# Путь к вашему файлу JSON
input_filename = 'kaikki.org-dictionary-all.json'
output_filename = 'result_structure_optional.json'

result_structure = process_file(input_filename)
write_to_file(result_structure, output_filename)

print(f"Результат был записан в файл: {output_filename}")
