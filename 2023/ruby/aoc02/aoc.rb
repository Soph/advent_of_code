file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

sum = 0
max = { red: 12, green: 13, blue: 14}
lines.each do |line|
  game = line.split(": ")
  reveals = game[1].split("; ")
  to_big = false
  reveals.each do |reveal|
    break if to_big
    reveal.split(", ").map do |item|
      parts = item.split(" ")
      puts "#{parts[1]}: #{parts[0]} vs #{max[parts[1].to_sym]}"
      if max[parts[1].to_sym] < parts[0].to_i
        puts "to big!"
        to_big = true
        break
      end
    end
  end
  unless to_big
    sum += game[0].split(" ")[1].to_i
    puts " #{game[0].split(" ")[1].to_i} is possible"
  end
end

puts "Part 1: #{sum}"

power_sum = 0
lines.each do |line|
  max_used = { red: 0, green: 0, blue: 0}
  game = line.split(": ")
  reveals = game[1].split("; ")
  reveals.each do |reveal|
    reveal.split(", ").map do |item|
      parts = item.split(" ")
      if max_used[parts[1].to_sym] < parts[0].to_i
        max_used[parts[1].to_sym] = parts[0].to_i
      end
    end
  end
  power_sum += max_used[:red] * max_used[:green] * max_used[:blue]
end

puts "Part 2: #{power_sum}"