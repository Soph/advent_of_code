require "set"

file = File.open(ARGV[0])
numbers = file.readlines.map(&:chomp).map(&:to_i)

result = 0

def calc_secret(number)
  secret = (number ^ (number * 64)) % 16777216
  secret = (secret ^ (secret / 32)) % 16777216
  (secret ^ (secret * 2048)) % 16777216
end

result = 0
secrets = {}
numbers.each do |number|
  secrets[number] = [number % 10]
  secret = number
  2000.times do |i|
    secret = calc_secret(secret)
    secrets[number] << secret % 10
  end
  result += secret
end

puts "Part1: #{result}"

combinations = {}
diffs = {}
secrets.each do |number, secrets|
  diffs[number] = secrets.each_cons(2).map do |a,b|
    b - a
  end
  seen = Set.new
  diffs[number].each_cons(4).each_with_index do |a,i|
    next if seen.include?(a) # we only want the first
    seen << a
    combinations[a] ||= []
    combinations[a] << secrets[i+4]
  end
end

puts "Part2: #{combinations.map{|k,v| v.sum}.max}"