import Foundation

let data = try String(contentsOfFile: "data.txt", encoding: .utf8)

let values = data.split(separator: "\n")
let intValues = values.compactMap { Int($0) }

func twoNumbers(values: [Int]) {
    for i in 0 ... (values.count - 1) {
        for j in 0 ... (values.count - 1) {
            if (i == j) {
                continue
            }
            let sum = values[i] + values[j]
            if (sum == 2020) {
                print("The two numbers are \(values[i]) and \(values[j]) and the result is \(values[i] * values[j])")
                return;
            }
        }
    }
}

func threeNumbers(values: [Int]) {
    for i in 0 ... (values.count - 1) {
        for j in 0 ... (values.count - 1) {
            for k in 0 ... (values.count - 1) {
                if (i == j || i == k || j == k) {
                    continue
                }
                let sum = values[i] + values[j] + values[k]
                if (sum == 2020) {
                    print("The three numbers are \(values[i]),  \(values[j]) and \(values[k]) and the result is \(values[i] * values[j] * values[k])")
                    return;
                }
            }
        }
    }
}

twoNumbers(values: intValues)
threeNumbers(values: intValues)

