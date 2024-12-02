file = File.open(ARGV[0])
reports = file.readlines.map(&:chomp).map{|line| line.split(" ").map(&:to_i)}

safes_1 = []
safe_count = 0
reports.each do |report|
  safe = true
  next if report[0] == report[1] # equal isn't allowed either

  up = report[0] < report[-1]
  last = report[0]
  report[1..].each do |number|
    if number < last && up
      safe = false
      break
    end
    if number > last && !up
      safe = false
      break
    end
    if ![1,2,3].include?((number-last).abs)
      safe = false
      break
    end
    last = number
  end

  if safe
    safe_count += 1
    safes_1 << report
  end
end

puts "Part1: #{safe_count}"

safes_2 = []
safe_count = 0

def failures(report)
  failures = 0
  up = report[0] < report[-1]
  last = report[0]

  report[1..].each do |number|
    break if failures > 1
    if last == number
      failures += 1
      next
    end
    if number < last && up
      failures += 1
      next
    end
    if number > last && !up
      failures += 1
      next
    end
    if ![1,2,3].include?((number-last).abs)
      failures += 1
      next
    end
    last = number
  end

  failures
end
reports.each do |report|
  if failures(report) <= 1
    safe_count += 1
    safes_2 << report
  else
    # algo can't handle the first being the one to drop, so let's retry without the first
    if failures(report[1..]) == 0
      safe_count += 1
      safes_2 << report 
    end
  end
end

puts "Part2: #{safe_count}"

#puts safes_2.inspect