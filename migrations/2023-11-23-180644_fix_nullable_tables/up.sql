-- Your SQL goes here

-- Update columns in words table
ALTER TABLE words ALTER COLUMN pos SET NOT NULL;
ALTER TABLE words ALTER COLUMN word SET NOT NULL;
ALTER TABLE words ALTER COLUMN lang_code SET NOT NULL;
ALTER TABLE words ALTER COLUMN lang SET NOT NULL;

-- Update columns in forms table
ALTER TABLE forms ALTER COLUMN word_id SET NOT NULL;
ALTER TABLE forms ALTER COLUMN form SET NOT NULL;

-- Update columns in tags table
ALTER TABLE tags ALTER COLUMN form_id SET NOT NULL;
ALTER TABLE tags ALTER COLUMN tag SET NOT NULL;

-- Update columns in etymology_templates table
ALTER TABLE etymology_templates ALTER COLUMN word_id SET NOT NULL;
ALTER TABLE etymology_templates ALTER COLUMN expansion SET NOT NULL;
ALTER TABLE etymology_templates ALTER COLUMN name SET NOT NULL;

-- Update columns in args table
ALTER TABLE args ALTER COLUMN etymology_template_id SET NOT NULL;
ALTER TABLE args ALTER COLUMN arg_key SET NOT NULL;
ALTER TABLE args ALTER COLUMN arg_val SET NOT NULL;

-- Update columns in sounds table
ALTER TABLE sounds ALTER COLUMN word_id SET NOT NULL;

-- Update columns in sound_tags table
ALTER TABLE sound_tags ALTER COLUMN sound_id SET NOT NULL;
ALTER TABLE sound_tags ALTER COLUMN tag SET NOT NULL;

-- Update columns in hyphenations table
ALTER TABLE hyphenations ALTER COLUMN word_id SET NOT NULL;
ALTER TABLE hyphenations ALTER COLUMN hyphen SET NOT NULL;

-- Update columns in hyphenations table
ALTER TABLE derived_words ALTER COLUMN word_id SET NOT NULL;
ALTER TABLE derived_words ALTER COLUMN word SET NOT NULL;
ALTER TABLE derived_words ALTER COLUMN _dis1 SET NOT NULL;

-- Update columns in senses table
ALTER TABLE senses ALTER COLUMN word_id SET NOT NULL;
ALTER TABLE senses ALTER COLUMN id_1 SET NOT NULL;

-- Update columns in links table
ALTER TABLE links ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE links ALTER COLUMN link_1 SET NOT NULL;
ALTER TABLE links ALTER COLUMN link_2 SET NOT NULL;

-- Update columns in links table
ALTER TABLE links ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE links ALTER COLUMN link_1 SET NOT NULL;
ALTER TABLE links ALTER COLUMN link_2 SET NOT NULL;

-- Update columns in raw_glosss table
ALTER TABLE raw_glosss ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE raw_glosss ALTER COLUMN raw_gloss SET NOT NULL;

-- Update columns in glosss table
ALTER TABLE glosss ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE glosss ALTER COLUMN gloss SET NOT NULL;

-- Update columns in sense_tags table
ALTER TABLE sense_tags ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE sense_tags ALTER COLUMN tag SET NOT NULL;

-- Update columns in categorys table
ALTER TABLE categorys ALTER COLUMN sense_id SET NOT NULL;
ALTER TABLE categorys ALTER COLUMN name SET NOT NULL;
ALTER TABLE categorys ALTER COLUMN kind SET NOT NULL;
ALTER TABLE categorys ALTER COLUMN source SET NOT NULL;

-- Update columns in category_parents table
ALTER TABLE category_parents ALTER COLUMN category_id SET NOT NULL;
ALTER TABLE category_parents ALTER COLUMN parent SET NOT NULL;

