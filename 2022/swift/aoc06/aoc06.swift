import Foundation

let data = try String(contentsOfFile: "data.txt", encoding: .utf8)

func find_uniq(input: String, len: Int) {
    var buffer: [Character] = []
    var index = 0

    for c in Array(input) {
        index += 1
        buffer.append(c)
        if buffer.count > len {
            buffer.remove(at: 0)
        }

        if buffer.count == len && Set(buffer).count == len {
            print("Result: \(index)")
            break
        }
    } 
}

find_uniq(input: data, len: 4)
find_uniq(input: data, len: 14)