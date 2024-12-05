file = File.open(ARGV[0])
sections = file.readlines.map(&:chomp).join("\n").split("\n\n")


rules = {}
sections[0].split("\n").map{|rule| rule.split("|")}.each do |rule|
  rules[rule[0].to_i] ||= [] 
  rules[rule[0].to_i] << rule[1].to_i
end

pages = sections[1].split("\n").map{|rule| rule.split(",").map(&:to_i) }

def valid?(page, rules)
  page.each_with_index do |number, index|
    next if rules[number].nil?

    return false unless rules[number].all?{ |num| page.index(num).nil? || page.index(num) > index }
  end
  
  true
end

valids = []
invalids = []
pages.each do |page|
  if valid?(page, rules)
    valids << page
  else
    invalids << page
  end
end

puts "Part1: #{valids.sum{|valid| valid[valid.size/2]}}"

new_valids = []
invalids.each do |page|
  fixed = page.clone
  while !valid?(fixed, rules) do
    fixed.each_with_index do |number, index|
      next if rules[number].nil?

      positions = rules[number].map{ |num| fixed.index(num) }.compact.sort
      if positions.size > 0 && positions[0] < index
        fixed.delete_at(index)
        fixed.insert(positions[0], number)
        break
      end
    end
  end
  new_valids << fixed
end

puts "Part1: #{new_valids.sum{|valid| valid[valid.size/2]}}"