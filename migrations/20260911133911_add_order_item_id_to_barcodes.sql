-- Add migration script here
ALTER TABLE barcodes
ADD COLUMN order_item_id UUID REFERENCES order_items (id) ON DELETE SET NULL;

CREATE INDEX idx_barcodes_order_item_id ON barcodes (order_item_id);