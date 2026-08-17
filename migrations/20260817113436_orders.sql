-- Add migration script here
CREATE TYPE order_status AS ENUM (
    'pending',
    'confirmed',
    'processing',
    'shipped',
    'delivered',
    'cancelled',
    'refunded',
    'walkin'
);

CREATE TABLE
    orders (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_number VARCHAR(30) NOT NULL,
        customer_name VARCHAR(150) NOT NULL,
        customer_email VARCHAR(150),
        customer_phone VARCHAR(30),
        shipping_address TEXT,
        status order_status NOT NULL DEFAULT 'pending',
        subtotal NUMERIC(12, 2) NOT NULL DEFAULT 0,
        tax_amount NUMERIC(12, 2) NOT NULL DEFAULT 0,
        shipping_amount NUMERIC(12, 2) NOT NULL DEFAULT 0,
        total_amount NUMERIC(12, 2) NOT NULL DEFAULT 0,
        notes TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT uq_orders_order_number UNIQUE (order_number),
        CONSTRAINT ck_orders_amounts_non_negative CHECK (
            subtotal >= 0
            AND tax_amount >= 0
            AND shipping_amount >= 0
            AND total_amount >= 0
        )
    );

CREATE INDEX idx_orders_status ON orders (status);

CREATE INDEX idx_orders_created_at ON orders (created_at);

CREATE INDEX idx_orders_customer_email ON orders (customer_email);

CREATE TABLE
    order_items (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        order_id UUID NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
        product_id UUID NOT NULL REFERENCES products (id) ON DELETE RESTRICT,
        -- snapshot fields: preserve what was ordered even if the product later changes/is deleted
        product_name VARCHAR(150) NOT NULL,
        product_sku VARCHAR(50) NOT NULL,
        unit_price NUMERIC(12, 2) NOT NULL,
        quantity INTEGER NOT NULL,
        line_total NUMERIC(12, 2) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        CONSTRAINT ck_order_items_quantity_positive CHECK (quantity > 0),
        CONSTRAINT ck_order_items_amounts_non_negative CHECK (
            unit_price >= 0
            AND line_total >= 0
        )
    );

CREATE INDEX idx_order_items_order_id ON order_items (order_id);

CREATE INDEX idx_order_items_product_id ON order_items (product_id);