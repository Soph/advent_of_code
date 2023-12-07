file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

class Hand
  def initialize(cards, bid)
    @cards = cards
    @bid = bid
  end

  def cards
    @cards
  end

  def bid
    @bid
  end

  def find_max_variant(hand)
    return hand if hand !~ /J/ # no joker
    variants = [hand]
    hand.chars.map.with_index{|c, i| c == "J" ? i : nil}.compact.each do |i|
      variants.clone.each do |variant_hand|
        ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].each do |replace|
          chars = variant_hand.chars
          chars[i] = replace
          variants << chars.join
        end
      end
    end
    sorted = variants.sort do |a,b|
      type(a) <=> type(b)
    end

    sorted.last
  end  

  def max_variant
    @max_variant ||= find_max_variant(@cards)
  end

  def max_variant_cards
    max_variant.chars
  end
end

hands = lines.map{ |line| line.split(" ") }.map{|hand| Hand.new(hand[0], hand[1].to_i)}

def type(hand)
  chars = hand.chars.tally.sort_by {|k,v| v }
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
    1
  when 'T'
    10
  else 
    card.to_i
  end
end

def compare_hand(a, b)
  (0..5).each do |i|
    if card_type(a.cards[i]) != card_type(b.cards[i])  
      return card_type(a.cards[i]) <=> card_type(b.cards[i])
    end
  end
end

def sort_hands(hands)
  hands.sort do |a,b|
    if type(a.max_variant) != type(b.max_variant)
      type(a.max_variant) <=> type(b.max_variant)
    else
      compare_hand(a, b) # compare original cards here
    end
  end
end

sorted = sort_hands(hands)

result = 0
sorted.each_with_index do |hand, i|
  result += (i+1)*hand.bid
end
puts result