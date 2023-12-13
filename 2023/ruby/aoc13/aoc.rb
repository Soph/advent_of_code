file = File.open(ARGV[0])
areas = file.read.split("\n\n")

def rotate(area)
  new_area = []
  area.split("\n").each_with_index do |line, y|
    line.chars.each_with_index do |char, x|
      new_area << [] if new_area[x].nil?
      new_area[x] << char
    end
  end
  
  new_area.map{|line| line.reverse.join }.join("\n")
end

rotated_areas = areas.map{|area| rotate(area)}

def find_mirror_line(area)
  lines = area.split("\n")
  results = []
  lines.each_with_index do |line, i|
    if line == lines[i+1]
      #puts "FOUND: #{i}: #{line} . #{lines.size}"
      if i >= (lines.size) / 2
        # puts lines[1..i]
        #puts "A vs"
        # puts lines[(i+1)..].reverse
        start_pos = i - (lines.size - (i+1)) + 1
        #puts "Start: #{start_pos}"
        if lines[start_pos..i] == lines[(i+1)..].reverse
          #puts "MATCH: #{i}"
          #puts "Distance: #{lines.size} : #{i} : #{lines.size - (i+1)}"
          results << i+1
        end
      else
        # puts lines[0..i]
        #puts "B #{i}"
        # puts lines[(i+1)..-2].reverse
        end_pos = i + 1 + i
        #puts "End: #{end_pos}"
        if lines[0..i] == lines[(i+1)..end_pos].reverse
          #puts "MATCH: #{i}"
          results << i+1
        end
      end
    end
  end
  results
end

vert = areas.map{|area| (find_mirror_line(area).first || 0) }
hori = rotated_areas.map{|area| find_mirror_line(area).first || 0}
sum = vert.sum * 100
sum += hori.sum 

puts "Part1: #{sum}"

def alternate(area)
  area_array = area.split("\n").map{|line| line.chars}

  (0...area_array.size).each do |y|
    (0...area_array[0].size).each do |x|
      area_array[y][x] = area_array[y][x] == '#' ? '.' : '#'
      new_area = area_array.map {|line| line.join }.join("\n")
      area_array[y][x] = area_array[y][x] == '#' ? '.' : '#'
      #puts new_area
      #puts
      find_mirror_line(new_area).each do |a|
        #puts "A: #{a} (#{x},#{y})" if a > 0
        if a >= (area_array.size) / 2
          # bottom
          return a * 100 if y > area_array.size - 2 * (area_array.size - a) && y < (2 * a) - 1
        else
          # top
          return a * 100 if y < (2 * a)
        end
      end
      find_mirror_line(rotate(new_area)).each do |b|
        #puts "B: #{b} (#{x},#{y}) #{(area_array[0].size) / 2}" if b > 0
        if b >= (area_array[0].size) / 2
          # right          
          return b if x > area_array[0].size - 2 * (area_array[0].size - b) && x < (2 * b) - 1
        else
          # left
          return b if x < (2 * b)
        end
      end
    end
  end
  return 0
end

#puts alternate(areas[0])
#puts areas.map{|area| alternate(area)}.inspect
puts "Part2: #{areas.map{|area| alternate(area)}.sum}"