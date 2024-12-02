file = File.open(ARGV[0])
reports = file.readlines.map(&:chomp).map{|line| line.split(" ").map(&:to_i)}

def valid?(report)
  return false if report[0] == report[1] # equal isn't allowed either

  up = report[0] < report[-1]
  last = report[0]
  report[1..].each do |number|
    return false if number < last && up
    return false if number > last && !up
    return false if ![1,2,3].include?((number-last).abs)
    last = number
  end

  true
end

safe_count = 0
reports.each do |report|
  safe_count += 1 if valid?(report)
end

puts "Part1: #{safe_count}"

safe_count = 0
reports.each do |report|
  if valid?(report)
    safe_count += 1
    next
  end
  report.size.times do |i|
    if valid?(report[0...i] + report[i+1...])
      safe_count += 1
      break
    end
  end
end

puts "Part2: #{safe_count}"
