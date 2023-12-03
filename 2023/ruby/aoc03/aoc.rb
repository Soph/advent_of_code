file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

schematic = lines.map {|line| line.chars }
sum = 0

current = ""
symbol = false
schematic.each_with_index do |items, y|
  unless current == ""
    sum += current.to_i if symbol
  end
  current = ""
  symbol = false
  items.each_with_index do |number, x|
    if number =~ /[0-9]/ # number
      current << number
      (x-1..x+1).each do |xn|
        (y-1..y+1).each do |yn|
          next if xn == x && yn == y
          next if schematic[yn].nil?
          next if schematic[yn][xn].nil?
          symbol = true if schematic[yn][xn] !~ /[0-9]/ && schematic[yn][xn] != '.'
        end
      end
    else
      unless current == ""
        sum += current.to_i if symbol
      end
      current = ""
      symbol = false
    end
  end
end

puts "Part1: #{sum}"

def extract_number(schematic, y, x)
  xstart = x
  while !schematic[y][xstart-1].nil? && schematic[y][xstart-1] =~ /[0-9]/
    xstart -= 1
  end

  number = ""
  size = 0
  while (schematic[y][xstart+size] =~ /[0-9]/)
    number << schematic[y][xstart+size]
    size += 1
  end

  number.to_i
end

sum = 0
schematic.each_with_index do |items, y|
  items.each_with_index do |number, x|
    if number == "*"
      numbers = []
      (x-1..x+1).each do |xn|
        (y-1..y+1).each do |yn|
          next if xn == x && yn == y
          next if schematic[yn].nil?
          next if schematic[yn][xn].nil?
          next if schematic[yn][xn] !~ /[0-9]/

          numbers << extract_number(schematic, yn, xn)
        end
      end
      sum += numbers.uniq.inject(:*) if numbers.uniq.size == 2
    end
  end
end

puts "Part2: #{sum}"