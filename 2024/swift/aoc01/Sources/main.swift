import Foundation

let data = try String(contentsOfFile: "data.txt", encoding: .utf8)

let lines = data.split(separator: "\n")
var left = [Int]()
var right = [Int]()

lines.forEach { line  in
  let items = line.split(separator: " ")
  left.append(Int(items[0]) ?? 0)
  right.append(Int(items[1]) ?? 0)
}

left.sort()
right.sort()

var distances = [Int]()

for (index, item) in left.enumerated() {
  distances.append(abs(item - right[index]))
}

print("Part1: \(distances.reduce(0, +))")

let counts = right.reduce(into: [:]) { counts, number in
  counts[number, default: 0] += 1
}

var times = [Int]()
left.forEach { value in
  if counts.contains(where: { $0.key == value }) {
    times.append(value * (counts[value] ?? 0))
  }  
}

print("Part2: \(times.reduce(0, +))")