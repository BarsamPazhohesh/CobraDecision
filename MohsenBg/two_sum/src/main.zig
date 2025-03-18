const std = @import("std");
const ArrayList = std.ArrayList;
const Allocator = std.mem.Allocator;

fn twoSum(allocator: Allocator, nums: []const i32, target: i32) !?[2]usize {
    var map = std.AutoHashMap(i32, usize).init(allocator);
    defer map.deinit();

    for (nums, 0..) |num, i| {
        const diff = target - num;

        if (map.get(diff)) |index| {
            return .{ index, i };
        }

        try map.put(num, i);
    }

    return null;
}

const testing = std.testing;
pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const nums = [4]i32{ 2, 7, 11, 15 };
    const target = 9;

    if (try twoSum(allocator, &nums, target)) |result| {
        std.debug.print("Indices: [{}, {}]\n", .{ result[0], result[1] });
    } else {
        std.debug.print("No solution found\n", .{});
    }
}

const test_allocator = std.testing.allocator;

test "twoSum first and last element" {
    const nums = [_]i32{ 1, 2, 3, 4, 5, 9 };
    const target = 10;
    const expected = [_]usize{ 0, 5 }; // 1 + 9 = 10

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum consecutive elements" {
    const nums = [_]i32{ 10, 20, 30, 40 };
    const target = 50;
    const expected = [_]usize{ 1, 2 }; // 20 + 30 = 50

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum with only two elements" {
    const nums = [_]i32{ 5, 25 };
    const target = 30;
    const expected = [_]usize{ 0, 1 }; // 5 + 25 = 30

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum with negative and positive numbers" {
    const nums = [_]i32{ -5, 10, 15, -10 };
    const target = 5;
    const expected = [_]usize{ 0, 1 }; // -5 + 10 = 5

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum with repeated numbers" {
    const nums = [_]i32{ 3, 3, 4, 5 };
    const target = 6;
    const expected = [_]usize{ 0, 1 }; // 3 + 3 = 6

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum with large numbers" {
    const nums = [_]i32{ 1000000, 500000, 300000, 700000 };
    const target = 1300000;
    const expected = [_]usize{ 0, 2 }; // 300000 + 1000000 = 1300000

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum no solution with negatives" {
    const nums = [_]i32{ -10, -20, -30, -40 };
    const target = -100; // No two numbers sum to -100

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result == null);
}

test "twoSum with all zeroes" {
    const nums = [_]i32{ 0, 0, 0, 0 };
    const target = 0;
    const expected = [_]usize{ 0, 1 }; // 0 + 0 = 0

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum with mixed values" {
    const nums = [_]i32{ -3, 1, 4, -2, 6, 8 };
    const target = 2;
    const expected = [_]usize{ 2, 3 }; // 4 + (-2) = 2

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}

test "twoSum where target is sum of same index" {
    const nums = [_]i32{ 1, 2, 3, 4, 5 };
    const target = 4;
    const expected = [_]usize{ 0, 2 }; // 1 + 3 = 4

    const result = try twoSum(test_allocator, &nums, target);
    try testing.expect(result != null);
    try testing.expectEqualSlices(usize, &expected, &result.?);
}
