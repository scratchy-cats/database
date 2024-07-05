const std = @import("std");
const math = std.math;
const Murmur = std.hash.Murmur3_32;
const Allocator = std.mem.Allocator;
const BitArray = @import("bit-array.zig").BitArray;

const BloomFilterError = error{ InvalidFalsePositiveProbability, UnsupportedHashFunctionCount };

const BloomFilter = struct {
    bitArray: BitArray,
    hashFunctionCount: u8,

    pub fn init(memoryAllocator: Allocator, maxItemCount: u64, falsePositiveProbability: f32) !BloomFilter {
        if ((falsePositiveProbability <= 0) or (falsePositiveProbability >= 1))
            return BloomFilterError.InvalidFalsePositiveProbability;

        const bitArraySize = calculateBitArraySize(maxItemCount, falsePositiveProbability);

        const hashFunctionCount = calculateHashFuncitonCount(maxItemCount, bitArraySize);
        // hashFunctionCount cannot be greater than 256.
        if (hashFunctionCount > math.maxInt(u8))
            return BloomFilterError.UnsupportedHashFunctionCount;

        return BloomFilter{ .bitArray = try BitArray.init(memoryAllocator, bitArraySize), .hashFunctionCount = @truncate(hashFunctionCount) };
    }

    pub fn deinit(self: *BloomFilter) void {
        self.bitArray.deinit();
    }

    pub fn insert(self: *BloomFilter, item: []const u8) !void {
        for (0..self.hashFunctionCount) |i| {
            const bitArrayIndex = self.getBitArrayIndex(item, @truncate(i));
            try self.bitArray.setBit(bitArrayIndex);
        }
    }

    pub fn contains(self: *BloomFilter, item: []const u8) !bool {
        for (0..self.hashFunctionCount) |i| {
            const bitArrayIndex = self.getBitArrayIndex(item, @truncate(i));
            if (try self.bitArray.getBit(bitArrayIndex) == 0)
                return false;
        }

        return true;
    }

    // For the given element, caluclates the target index in the bit array, using the Murmur hash
    // function.
    fn getBitArrayIndex(self: *BloomFilter, item: []const u8, seed: u32) u64 {
        const hash = Murmur.hashWithSeed(item, seed);
        return hash % self.bitArray.size;
    }
};

// Given the maximum number of items we can insert (n) and the target probability of False Positives
// (fp), it caluclates and returns the size of the bit array (m).
// NOTE : m = - n.ln(fp) / ln(2)^2.
fn calculateBitArraySize(maxItemCount: u64, falsePositiveProbability: f32) u64 {
    const numerator = -@as(f64, @floatFromInt(maxItemCount)) * math.log(f64, math.e, falsePositiveProbability);
    const denominator = math.pow(f64, math.log(f64, math.e, 2), 2);

    return @as(u64, @intFromFloat(math.divTrunc(f64, numerator, denominator) catch unreachable));
}

// Given the maximum number of items we can insert (n) and the size of the bit array (m), it
// caluclates and returns the optimal hash functions count (k).
// NOTE : k = m.ln(2) / n.
fn calculateHashFuncitonCount(maxItemCount: u64, bitArraySize: u64) u64 {
    const numerator = @as(f64, @floatFromInt(bitArraySize)) * math.log(f64, math.e, 2);
    const denominator = @as(f64, @floatFromInt(maxItemCount));

    return @as(u64, @intFromFloat(math.divTrunc(f64, numerator, denominator) catch unreachable));
}

pub fn main() !void {
    var bloomFilter = try BloomFilter.init(std.heap.page_allocator, 1000, 0.1);
    defer bloomFilter.deinit();

    try bloomFilter.insert("archi");

    std.debug.print("{}", .{try bloomFilter.contains("adam")});
}
