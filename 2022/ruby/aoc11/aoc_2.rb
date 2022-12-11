file = File.open(ARGV[0])
raw_monkeys = file.readlines.map(&:chomp)

def parse_monkeys(raw_monkeys)
  monkeys = []
  raw_monkeys.each_slice(7) do |raw_monkey|
    monkey = {}

    monkey[:number] = raw_monkey[0].chomp.match(/Monkey (\d*):/)[1].to_i
    monkey[:items] = raw_monkey[1].chomp.match(/Starting items: (.*)/)[1].split(", ").map(&:to_i)
    operation = raw_monkey[2].chomp.match(/Operation: new = old (.*) (.*)/)
    monkey[:operation] = {operator: operation[1], number: operation[2]}
    monkey[:test] = raw_monkey[3].chomp.match(/Test: divisible by (\d*)/)[1].to_i
    monkey[:if_true_monkey] = raw_monkey[4].chomp.match(/If true: throw to monkey (\d)/)[1].to_i
    monkey[:if_false_monkey] = raw_monkey[5].chomp.match(/If false: throw to monkey (\d)/)[1].to_i

    monkeys << monkey
  end
  monkeys
end

monkeys = parse_monkeys(raw_monkeys)
monkey_inspect = monkeys.length.times.map {0}

biggest_common_divider = monkeys.map{|m| m[:test]}.inject(&:*)

10000.times do |round|
  monkeys.length.times do |i|
    monkey = monkeys[i]
    monkey[:items].each do |item|
      monkey_inspect[i] += 1
      op = monkey[:operation][:operator]
      if monkey[:operation][:number] == "old"
        new_worry = op == "*" ? item * item : item + item
      else
        number = monkey[:operation][:number].to_i
        new_worry = op == "*" ? item * number : item + number
      end
      new_worry %= biggest_common_divider
      if new_worry % monkey[:test] == 0
        #puts "Monkey #{i}: #{item} -> #{new_worry} test true throw to #{monkey[:if_true_monkey]}"
        monkeys[monkey[:if_true_monkey]][:items] << new_worry
      else
        #puts "Monkey #{i}: #{item} -> #{new_worry} test false throw to #{monkey[:if_false_monkey]}"
        monkeys[monkey[:if_false_monkey]][:items] << new_worry
      end
      #sleep(1) if monkey[:operation][:number] == "old"
    end
    monkey[:items] = []
  end
  # puts "Round #{round}"
  monkeys.map{|monkey| puts "Monkey #{monkey[:number]}: #{monkey[:items].join(", ")}"}
end

puts "Monkey Business2: #{monkey_inspect.max(2).inject(:*)}"
