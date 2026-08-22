-- Add migration script here
CREATE UNIQUE INDEX uq_images_category_id ON images (category_id)
WHERE
    category_id IS NOT NULL;

CREATE UNIQUE INDEX uq_images_sub_category_id ON images (sub_category_id)
WHERE
    sub_category_id IS NOT NULL;

CREATE UNIQUE INDEX uq_images_brand_id ON images (brand_id)
WHERE
    brand_id IS NOT NULL;