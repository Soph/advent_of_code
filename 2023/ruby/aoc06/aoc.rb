file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

times = lines[0].split(" ").map(&:chomp)[1..-1].map(&:to_i)
distances = lines[1].split(" ").map(&:chomp)[1..-1].map(&:to_i)

result = 1
times.each_with_index do |max_time, i|
  wins = 0
  (0..max_time).each do |time|
    #puts "#{time} * #{max_time - time} > #{distances[i]}"
    wins += 1 if time * (max_time - time) > distances[i]
  end
  #puts wins
  result *= wins
end

puts "Part1: #{result}"

times = [lines[0].split(" ").map(&:chomp)[1..-1].join.to_i]
distances = [lines[1].split(" ").map(&:chomp)[1..-1].join.to_i]

result = 1
times.each_with_index do |max_time, i|
  wins = 0
  (0..max_time).each do |time|
    #puts "#{time} * #{max_time - time} > #{distances[i]}"
    wins += 1 if time * (max_time - time) > distances[i]
  end
  #puts wins
  result *= wins
end

puts "Part2: #{result}"

