file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

cards = []
sum = 0
lines.each do |line|
  buckets = line.split(": ")[1].split(" | ")
  winning = buckets[0].split(" ")
  numbers = buckets[1].split(" ")
  wins = numbers & winning
  cards << [winning, numbers, wins.size]
  next if wins.size == 0
  sum += 2**(wins.size-1)
end

puts "Part1: #{sum}"

owned = {}
cards.each_with_index do |card, i|
  card_number = i+1
  owned[card_number] ||= 0
  owned[card_number] += 1
  (1...(card[2]+1)).each do |n|
    owned[card_number+n] ||= 0
    owned[card_number+n] += owned[card_number]
  end
end

puts "Part2: #{owned.values.sum}"