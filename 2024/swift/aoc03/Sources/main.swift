import Foundation
import RegexBuilder
let data = try String(contentsOfFile: "data.txt", encoding: .utf8)

if #available(macOS 13, *) {
  let firstNumber = Reference(Int.self)
  let secondNumber = Reference(Int.self)

  let search1 = Regex {
      "mul("

      TryCapture(as: firstNumber) {
        Capture {
          OneOrMore(.digit)
        }
      } transform: { match in
        Int(match)
      }

      ","

      TryCapture(as: secondNumber) {
        Capture {
          OneOrMore(.digit)
        }
      } transform: { match in
        Int(match)
      }

      ")"
  }
  var result = 0
  for match in data.matches(of: search1) {
    result += match[firstNumber] * match[secondNumber]
  }
  print("Result1: \(result)")

  let doString = Reference(Substring.self)
  let doNotString = Reference(Substring.self)
  let search2 = Regex {
      ChoiceOf {
        Regex {
          "mul("

          TryCapture(as: firstNumber) {
            Capture {
              OneOrMore(.digit)
            }
          } transform: { match in
            Int(match)
          }

          ","

          TryCapture(as: secondNumber) {
            Capture {
              OneOrMore(.digit)
            }
          } transform: { match in
            Int(match)
          }

          ")"
        }
        Capture(as: doString) {
          "do()"
        }
        Capture(as: doNotString) {
          "don't()"
        }
      }
      
  }
  result = 0
  var enabled = true
  for match in data.matches(of: search2) {    
    if (match.0 == "do()") {
      enabled = true
    } else if (match.0 == "don't()") {
      enabled = false
    } else if (enabled) {
      result += match[firstNumber] * match[secondNumber]
    }
  }
  print("Result2: \(result)")  
}