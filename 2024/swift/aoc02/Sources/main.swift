import Foundation
let data = try String(contentsOfFile: "data.txt", encoding: .utf8)
let lines = data.split(separator: "\n")

func is_valid(report: [Int]) -> Bool {
  if report[0] == report[1] { return false }

  let up = report.first ?? 0 < report.last ?? 0
  var last_number = report[0]
  for number in report[1..<report.endIndex] {
    if number < last_number && up { return false }
    if number > last_number && !up { return false }
    
    let diff = abs(number - last_number)
    if diff > 3 || diff < 1 { return false }

    last_number = number
  }

  return true
}

var safe_count = 0
lines.forEach { line in
  let report = line.split(separator: " ").map { Substring in
    Int(Substring) ?? 0
  }
  if is_valid(report: report) {
    safe_count += 1
  }
}

print("Part1: \(safe_count)")

safe_count = 0
for line in lines {
  let report = line.split(separator: " ").map { Substring in
    Int(Substring) ?? 0
  }
  if is_valid(report: report) {
    safe_count += 1
    continue
  }
  for i in 0..<report.endIndex {
    var new_report = Array(report[0..<i])
    new_report.append(contentsOf: Array(report[i+1..<report.endIndex]))
    if is_valid(report: new_report) {
      safe_count += 1
      break
    }
  }
}
print("Part2: \(safe_count)")