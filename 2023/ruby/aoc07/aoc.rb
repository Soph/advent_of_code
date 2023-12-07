file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

hands = lines.map{ |line| line.split(" ") }.map { |card| [card[0], card[1].to_i] }

def type(hand)
  chars = hand[0].chars.tally.sort_by {|k,v| v }
  if chars.size == 1
    return 7 # 5 of a kind
  elsif chars.size == 2 && chars.last[1] == 4 # 4 of a kind
    return 6
  elsif chars.size == 2 # full house
    return 5
  elsif chars.size == 3 && chars.last[1] == 3 # 3 of a kind
    return 4
  elsif chars.last[1] == 2 && chars[-2][1] == 2 # 2 * 2 of a kind
    return 3
  elsif chars.last[1] == 2 # 2 of a kind
    return 2
  else
    return 1
  end
end

def card_type(card)
  case card
  when 'A'
    14
  when 'K'
    13
  when 'Q'
    12
  when 'J'
    11
  when 'T'
    10
  else 
    card.to_i
  end
end

def compare_hand(a, b)
  (0..5).each do |i|
    if card_type(a[0].chars[i]) != card_type(b[0].chars[i])  
      return card_type(a[0].chars[i]) <=> card_type(b[0].chars[i])
    end
  end
end

puts hands.inspect

sorted = hands.sort do |a,b|
  if type(a) != type(b)
    puts "1 #{a} : #{b}"
    type(a) <=> type(b)
  else
    puts "2 #{a} : #{b}"
    compare_hand(a, b)
  end
end

puts sorted.inspect

result = 0
sorted.each_with_index do |hand, i|
  puts "#{i+1}: #{hand[1]}"
  result += (i+1)*hand[1]
end
puts result