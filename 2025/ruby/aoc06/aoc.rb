file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

problems = []
lines.each_with_index do |line, _i|
  line.strip.split(/\s+/).each_with_index do |number, j|
    problems[j] ||= []
    problems[j] << number
  end
end

part1 = problems.map do |problem|
  problem[0..-2].map(&:to_i).inject(problem[-1].to_sym)
end.sum

puts "Part 1: #{part1}"

transformed = []
lines.each_with_index do |line, _y|
  line.split('').each_with_index do |place, x|
    transformed[x] ||= []
    transformed[x] << place
  end
end

operations = []
digits = []

block = 0
transformed.each do |part|
  joined = part[0..-2].join.strip
  if joined == ''
    block += 1
    next
  end
  digits[block] ||= []
  digits[block] << joined.to_i
  operations[block] = part[-1] if part[-1].strip != ''
end

puts "Part 2: #{digits.each_with_index.map { |digit, i| digit.inject(operations[i].to_sym) }.sum}"
