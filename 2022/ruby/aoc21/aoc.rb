require "set"
file = File.open(ARGV[0])
monkeys = file.readlines.map(&:chomp).map do |line|
  parts = line.split(": ")
  if /(?<left>.*) (?<operator>[\+\-\*\/]) (?<right>.*)/ =~ parts[1]
    parts[1] = {
      left: left,
      operator: operator.gsub("=", "=="),
      right:right
    }
  else
    parts[1] = parts[1].to_i
  end

  [parts[0], parts[1]]
end.to_h

#puts monkeys.inspect

def execute(monkeys, current)
  monkey = monkeys[current]
  if monkey.is_a?(Numeric)
    return monkey
  else
    return eval("execute(monkeys, monkey[:left]) #{monkey[:operator]} execute(monkeys, monkey[:right])")
  end
end

puts "Result1: #{execute(monkeys, "root")}"

monkeys2 = monkeys.clone

def invert(monkeys, monkeys2, current)
  humn_op_monkey = monkeys.each do |name, values|
    next if values.is_a?(Numeric)
    next unless values[:left] == current || values[:right] == current
  
    opposites = { "+" => "-", "-" => "+", "*" => "/", "/" => "*"}
    if values[:left] == current
      # ptdq: humn - dvpt
      monkeys2[current] = {
        left: name,
        operator: opposites[values[:operator]],
        right: values[:right]
      }
      monkeys2.delete(name)
      invert(monkeys, monkeys2, name)
    elsif values[:right] == current
      if values[:operator] == "+" || values[:operator] == "*"
        # lgvd: ljgn * ptdq
        monkeys2[current] = {
          left: name,
          operator: opposites[values[:operator]],
          right: values[:left]
        }
      else
        # pppw: cczh / lfqf
        monkeys2[current] = {
          left: values[:left],
          operator: values[:operator],
          right: name
        }
      end
      monkeys2.delete(name)
      invert(monkeys, monkeys2, name)
    end
  end
end

def execute2(monkeys, current)
  monkey = monkeys[current]
  if monkey.nil?
    raise
  end
  if monkey.is_a?(Numeric)
    return monkey
  else
    return eval("execute2(monkeys, monkey[:left]) #{monkey[:operator]} execute(monkeys, monkey[:right])")
  end
end

invert(monkeys, monkeys2, "humn")
# puts "Inverted: #{monkeys2.inspect}"

# Figure out which side of root is solvable
begin
  value = execute2(monkeys2, monkeys["root"][:left])
  monkeys2[monkeys["root"][:right]] = value
  puts "left: ok - #{monkeys["root"][:left]}: #{monkeys[monkeys["root"][:left]]} -> #{value}"
rescue 
end
begin
  value = execute2(monkeys2, monkeys["root"][:right])
  monkeys2[monkeys["root"][:left]] = value
  puts "right: ok - #{monkeys["root"][:right]}: #{monkeys[monkeys["root"][:right]]} -> #{value}"
rescue 
end

# Debug
# monkeys2.each do |name, value|
#   if value.is_a?(Numeric)
#     puts "#{name}: #{value}"
#   else
#     puts "#{name}: #{value[:left]} #{value[:operator]} #{value[:right]}"
#   end
# end

puts "Result2: #{execute2(monkeys2, "humn")}"
