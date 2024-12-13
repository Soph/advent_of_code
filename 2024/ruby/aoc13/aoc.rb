require "set"

file = File.open(ARGV[0])
machines_raw = file.readlines.map(&:chomp).join("\n").split("\n\n")

# # Button A: X+94, Y+34
# # Button B: X+22, Y+67
# # Prize: X=8400, Y=5400

# x1*a + x2*b = result_x
# y1*a + y2*b = result_y

# a = (result_x-x2*b)/x1
# y1*((result_x-x2*b)/x1) + y2*b = result_y
# y1*(result_x/x1-x2*b/x1) + y2*b = result_y
# (result_x*y1)/x1 - (x2*b*y1)/x1 + (y2*b*x1)/x1 = result_y 

# (result_x*y1)/x1 - ((x2*y1 + y2*x1)*b)/x1 = result_y
# (result_x*y1)/x1 + result_y = ((x2*y1 + y2*x1)*b)/x1
# ((result_x*y1) + result_y*x1))/(x2*y1 + y2*x1) = (b)

# x1*a + x2*b = r
# 34*a + 67*b = 5400

machines = []
machines_raw.each do |raw|
  machine = {}
  parts = raw.split("\n")
  match = parts[0].match(/X\+(\d*), Y\+(\d*)/)
  machine[:button_a] = [match[1].to_i, match[2].to_i]
  match = parts[1].match(/X\+(\d*), Y\+(\d*)/)
  machine[:button_b] = [match[1].to_i, match[2].to_i]
  match = parts[2].match(/X\=(\d*), Y\=(\d*)/)
  machine[:prize] = [match[1].to_i, match[2].to_i]
  machines << machine
end

valid = {}
machines.each do |machine|
  result_x, result_y = machine[:prize]
  x1, y1 = machine[:button_a]
  x2, y2 = machine[:button_b]
  times_b = (result_x * y1 - result_y * x1)/(x2 * y1 - y2 * x1)
  times_a = (result_x - x2 * times_b) / x1
  # check if no rounding at work
  if result_x == x1*times_a+x2*times_b && result_y == y1*times_a+y2*times_b
    valid[machine] = [times_a, times_b]
  end
end

result_sum = 0
valid.each do |machine, result|
  result_sum += result[0]*3 + result[1]*1
end

puts "Part1: #{result_sum}"

valid = {}
machines.each do |machine|
  result_x, result_y = machine[:prize]
  result_x += 10000000000000
  result_y += 10000000000000
  x1, y1 = machine[:button_a]
  x2, y2 = machine[:button_b]
  times_b = (result_x * y1 - result_y * x1)/(x2 * y1 - y2 * x1)
  times_a = (result_x - x2 * times_b) / x1
  # check if no rounding at work
  if result_x == x1*times_a+x2*times_b && result_y == y1*times_a+y2*times_b
    valid[machine] = [times_a, times_b]
  end
end

result_sum = 0
valid.each do |machine, result|
  result_sum += result[0]*3 + result[1]*1
end

puts "Part2: #{result_sum}"
