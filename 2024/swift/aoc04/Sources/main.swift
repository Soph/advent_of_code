import Foundation
let data = try String(contentsOfFile: "data.txt", encoding: .utf8)
let letters = data.split(separator: "\n").map { $0.map { String($0) } }

func get(x: Int, y: Int) -> String {
  guard x >= 0, y >= 0, x < letters[0].count, y < letters.count else {
    return ""
  }
  
  return letters[y][x]
}

var count = 0
for y in 0..<letters.count {
  for x in 0..<letters[0].count {
    guard get(x: x, y: y) == "X" else {
      continue
    }
    if get(x:x+1,y:y)+get(x:x+2,y:y)+get(x:x+3,y:y) == "MAS" {
      // right
      count += 1
    }
    if get(x:x,y:y+1)+get(x:x,y:y+2)+get(x:x,y:y+3) == "MAS" {
      // down
      count += 1
    }
    if get(x:x-1,y:y)+get(x:x-2,y:y)+get(x:x-3,y:y) == "MAS" {
      // left
      count += 1
    }
    if get(x:x,y:y-1)+get(x:x,y:y-2)+get(x:x,y:y-3) == "MAS" {
      // up
      count += 1
    }
    if get(x:x+1,y:y+1)+get(x:x+2,y:y+2)+get(x:x+3,y:y+3) == "MAS" {
      // down right
      count += 1
    }
    if get(x:x+1,y:y-1)+get(x:x+2,y:y-2)+get(x:x+3,y:y-3) == "MAS" {
      // up right
      count += 1
    } 
    if get(x:x-1,y:y-1)+get(x:x-2,y:y-2)+get(x:x-3,y:y-3) == "MAS" {
      // up left
      count += 1
    }
    if get(x:x-1,y:y+1)+get(x:x-2,y:y+2)+get(x:x-3,y:y+3) == "MAS" {
      // down left
      count += 1
    }
  }
}

print("Part1: \(count)")

count = 0
for y in 0..<letters.count {
  for x in 0..<letters[0].count {
    guard get(x: x, y: y) == "A" else {
      continue
    }
    if get(x:x-1,y:y-1) == "M" && get(x:x+1,y:y+1) == "S" && get(x:x-1,y:y+1) == "M" && get(x:x+1,y:y-1) == "S" {
      // M S
      //  A
      // M S
      count += 1
    }
    if get(x:x-1,y:y-1) == "S" && get(x:x+1,y:y+1) == "M" && get(x:x-1,y:y+1) == "M" && get(x:x+1,y:y-1) == "S" {
      // S S
      //  A
      // M M
      count += 1
    }
    if get(x:x-1,y:y-1) == "S" && get(x:x+1,y:y+1) == "M" && get(x:x-1,y:y+1) == "S" && get(x:x+1,y:y-1) == "M" {
      // S M
      //  A
      // S M
      count += 1
    }
    if get(x:x-1,y:y-1) == "M" && get(x:x+1,y:y+1) == "S" && get(x:x-1,y:y+1) == "S" && get(x:x+1,y:y-1) == "M" {
      // M M
      //  A
      // S S
      count += 1
    }
  }
}

print("Part2: \(count)")

