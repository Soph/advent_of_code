require "set"
file = File.open(ARGV[0])
numbers = file.readlines.map(&:chomp).map(&:chars)
@trace = false
sum = 0
numbers.each do |digits|
  value = 0
  digits.reverse.each_with_index do |digit, n|
    digit = -1 if digit == '-'
    digit = -2 if digit == '='

    value += 5**n * digit.to_i
  end
  sum += value
end

result_digits = []
left = sum
(0..40).to_a.reverse.each do |i|
  next if sum / 5**i == 0 && result_digits.empty?
  try = left / 5**i
  left = left - (try * 5**i)
  result_digits << try
end

(0..result_digits.size).to_a.reverse.each do |i|
  digit = result_digits[i]
  next if (0..2).include?(digit)

  if digit == 3
    result_digits[i] = "="
    result_digits[i-1] += 1
  elsif digit == 4
    result_digits[i] = "-"
    result_digits[i-1] += 1
  elsif digit == 5
    result_digits[i] = "0"
    result_digits[i-1] += 1
  end
end
puts "Result: #{result_digits.join}"

