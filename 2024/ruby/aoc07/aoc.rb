file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

operations = {}
lines.map do |line|
  parts = line.split(":")
  numbers = parts[1].split(" ").map(&:to_i)
  operations[parts[0].to_i] = numbers
end

def add_or_sum(left, right)
  [(left * right), (left + right)]
end

correct = 0
operations.each do |key, values|
  results = [values[0]]
  values[1..].each do |value|
    results = results.map do |result|
      add_or_sum(result, value)
    end
    results.flatten!
  end
  results.each do |result|
    if result == key
      correct += key
      break
    end
  end
end

puts "Part1: #{correct}"

def add_or_sum_or_concat(left, right)
  [(left * right), (left + right), [left,right].join.to_i]
end

correct = 0
operations.each do |key, values|
  results = [values[0]]
  values[1..].each do |value|
    results = results.map do |result|
      add_or_sum_or_concat(result, value)
    end
    results.flatten!
  end
  results.each do |result|
    if result == key
      correct += key
      break
    end
  end
end

puts "Part2: #{correct}"