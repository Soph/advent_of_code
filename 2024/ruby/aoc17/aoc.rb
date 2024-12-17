require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

register = data[0].split("\n").map do |line|
  match = line.match(/Register (.*): (\d*)/)
  [match[1], match[2].to_i]
end.to_h

instructions = data[1].match(/Program: (.*)/)[1].split(",").map(&:to_i)

def parse_operand(operand, register)
  case operand
  when 0..3
    operand
  when 4
    register["A"]
  when 5
    register["B"]
  when 6
    register["C"]
  end
end

i = 0
output = []
while i < instructions.size
  opcode = instructions[i]
  operand = instructions[i+1]
  #puts "#{i}: #{opcode} #{operand} #{parse_operand(operand, register)}"
  case opcode
  when 0 # adv
    # division
    operand = parse_operand(operand, register)
    #puts "#{register["A"]} / (2 ** #{operand})"
    register["A"] = register["A"] / (2 ** operand)
    i += 2
  when 1 # bxl
    # bit xor
    register["B"] = register["B"] ^ operand
    i += 2
  when 2 # bst
    operand = parse_operand(operand, register)
    register["B"] = operand % 8
    i += 2
  when 3 # jnz
    if register["A"] > 0
      i = operand
    else
      i += 2
    end
  when 4 # bxc
    register["B"] = register["B"] ^ register["C"]
    i += 2
  when 5 # out
    operand = parse_operand(operand, register)
    output << (operand % 8).to_s
    i += 2
  when 6 # bdv
    operand = parse_operand(operand, register)
    register["B"] = register["A"] / (2 ** operand)
    i += 2
  when 7 # cdv
    operand = parse_operand(operand, register)
    register["C"] = register["A"] / (2 ** operand)
    i += 2
  end
  #puts register.inspect
  #sleep 0.1
end

puts "Part1: #{output.join(",")}"