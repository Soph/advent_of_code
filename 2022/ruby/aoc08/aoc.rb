file = File.open(ARGV[0])
field = file.readlines.map(&:chomp).map(&:chars).map{ |line| line.map(&:to_i) }

def check(field, x, y)
  # outside row
  return true if x == 0 || y == 0 || x == field[0].length-1 || y == field.length-1
  return true if field[y][..x-1].max < field[y][x]
  return true if field[y][x+1..].max < field[y][x]

  vertical = []
  (0..field.length-1).each do |check_y|
    vertical << field[check_y][x]
  end

  return true if vertical[..y-1].max < field[y][x]
  return true if vertical[y+1..].max < field[y][x]

  false
end

def calc_senic_score(field, x, y)
  return 0 if x == 0 || y == 0 || x == field[0].length-1 || y == field.length-1

  tree = field[y][x]
  score = []
  # outside row
  index = field[y][..x-1].reverse.find_index {|height| height >= tree}
  score << (index.nil? ? x : index + 1)

  index = field[y][x+1..].find_index {|height| height >= tree}
  score << (index.nil? ? field[0].length - 1 - x : index + 1)
  
  vertical = []
  (0..field.length-1).each do |check_y|
    vertical << field[check_y][x]
  end

  index = vertical[..y-1].reverse.find_index {|height| height >= tree}
  score << (index.nil? ? y : index + 1)

  index = vertical[y+1..].find_index {|height| height >= tree}
  score << (index.nil? ? field.length - 1 - y : index + 1)

  score.inject(:*)
end

count = 0
max = 0
field.length.times do |y|
  field[0].length.times do |x|
    count += 1 if check(field, x, y)
    score = calc_senic_score(field, x, y)
    max = [max, score].max
  end
end

puts "Result1: #{count}"
puts "Result2: #{max}"
