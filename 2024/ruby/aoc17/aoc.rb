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


def run(instructions, register)
  i = 0
  output = []
  while i < instructions.size
    opcode = instructions[i]
    operand = instructions[i+1]
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
      output << (operand % 8)
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
  end

  output
end

puts "Part1: #{run(instructions, register).join(",")}"

def to_input(three_bit_chunks)
  result = 0
  three_bit_chunks.each { |n| result = result * 8 + n }
  result
end

found_digits = [[]]
instructions.each_with_index do |instruction, n|
  new_digits = []
  found_digits.each do |digits|
    8.times do |i|
      next_try = (digits+[i]).map{|i| i.to_s(2).rjust(3, "0")}.join.to_i(2)
      register["A"] = next_try
      if run(instructions, register).reverse == instructions.reverse[0..n]
        new_digits << digits + [i]
        #break
      end
    end
    found_digits = new_digits
  end
end
valid = 0
found_digits.each do |digits|
  number = digits.map{|i| i.to_s(2).rjust(3, "0")}.join.to_i(2)
  register = {"A" => number, "B" => 0, "C" => 0}
  if run(instructions, register) == instructions
    valid = number if valid == 0 || valid > number
  end
end

puts "Part2: #{valid}"