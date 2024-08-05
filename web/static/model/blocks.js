/**
 * Backing store for blocks.
 */

var m_blocks = {};

export class Block {
    /**
     * Constructor.
     *
     * @param {any} id The block ID.
     * @param {string} name The layer name.
     * @param {string} device_type The device type (see devices.js).
     * @param {object} data The device data.
     */
    constructor(id, name, type, data) {
        this.id = id;
        this.name = name;
        this.type = type;
        this.data = data;
    }
}

/**
 * Get all of the blocks.
 *
 * @returns All of the blocks.
 */
export function get_all_blocks() {
    return m_blocks;
}

/**
 * Get a block.
 *
 * @param {any} id The block ID.
 * @returns The block, if it exists.
 */
export function get_block(id) {
    return m_blocks[id];
}

/**
 * Update a block.
 * @param {any} id The block ID.
 * @param {Block} block The block.
 */
export function update_block(id, block) {
    m_blocks[id] = block;
}

/**
 * Clear all the blocks.
 */
export function clear_blocks() {
    m_blocks = {};
}