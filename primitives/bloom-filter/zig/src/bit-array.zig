const std = @import("std");
const Allocator = std.mem.Allocator;

const BitArrayError = error{ UnsupportedMemoryByteSize, InvalidIndex };

pub const BitArray = struct {
    size: u64,
    bytes: []u8,
    memoryAllocator: Allocator,

    pub fn init(memoryAllocator: Allocator, size: u64) !BitArray {
        // Calculate how many bytes of memory is needed.
        const memoryByteSize: u64 =
            if (size % 8 > 0) ((size / 8) + 1) else (size / 8);
        if (memoryByteSize > std.math.maxInt(usize))
            return BitArrayError.UnsupportedMemoryByteSize;
        const bytes = try memoryAllocator.alloc(u8, memoryByteSize);
        @memset(bytes, 0);

        return BitArray{ .size = size, .bytes = bytes, .memoryAllocator = memoryAllocator };
    }

    pub fn deinit(self: *BitArray) void {
        self.memoryAllocator.free(self.bytes);
    }

    pub fn setBit(self: *BitArray, bitIndex: u64) !void {
        try self.isValidBitIndex(bitIndex);

        // Index of the byte (in the bytes array) containing the target bit.
        const byteIndex: usize = @truncate(bitIndex / 8);

        // Suppose the byte be 01001000. And we want to set the bit at the 3rd offset to 1.
        // We'll create a bit-mask : 00000100. And then apply the bit-mask to the byte using an XOR
        // operator, which'll modify the byte to 01001100.
        const bitOffsetInByte: u3 = @truncate(bitIndex % 8);
        const bitMask = @as(u8, 1) << bitOffsetInByte;

        self.bytes[byteIndex] |= bitMask;
    }

    pub fn getBit(self: *BitArray, bitIndex: u64) !u1 {
        try self.isValidBitIndex(bitIndex);

        // We'll fetch the byte containing the target bit.
        const byteIndex: usize = @truncate(bitIndex / 8);
        const byte = self.bytes[byteIndex];

        // Suppose the byte be 01001000. And we want to get value of the bit at the 3rd offset.
        // We'll create a bit-mask : 00000100. And then apply the bit-mask to the byte using an AND
        // operator.
        // The operation will give us 00000000. We'll then return the value of the bit at the 3rd
        // offset of this resultant byte.

        const bitOffsetInByte: u3 = @truncate(bitIndex % 8);
        const bitMask = @as(u8, 1) << bitOffsetInByte;

        return @truncate((byte & bitMask) >> bitOffsetInByte);
    }

    fn isValidBitIndex(self: *BitArray, bitIndex: u64) !void {
        if (bitIndex >= self.size)
            return BitArrayError.InvalidIndex;
    }
};
