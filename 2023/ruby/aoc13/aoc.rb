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
# rotated_areas.each do |area|
#   puts area
#   puts
# end

def find_mirror_line(area)
  lines = area.split("\n")
  found = -1
  lines.each_with_index do |line, i|
    if line == lines[i+1]
      #puts "FOUND: #{i}: #{line} . #{lines.size}"
      if i >= (lines.size) / 2
        # puts lines[1..i]
        # puts "A vs"
        # puts lines[(i+1)..].reverse
        start_pos = i - (lines.size - (i+1)) + 1
        #puts "Start: #{start_pos}"
        if lines[start_pos..i] == lines[(i+1)..].reverse
          #puts "MATCH: #{i}"
          #puts "Distance: #{lines.size} : #{i} : #{lines.size - (i+1)}"
          found = i if i > found
        end
      else
        # puts lines[0..i]
        # puts "B vs"
        # puts lines[(i+1)..-2].reverse
        end_pos = i + 1 + i
        #puts "End: #{end_pos}"
        if lines[0..i] == lines[(i+1)..end_pos].reverse
          #puts "MATCH: #{i}"
          found = i if i > found
        end
      end
    end
  end
  if found >= 0
    found += 1
    return found
    #puts "Found #{found}: #{lines[found]}"
  end
  0
end

vert = areas.map{|area| (find_mirror_line(area)) }
hori = rotated_areas.map{|area| find_mirror_line(area)}
sum = vert.sum * 100
sum += hori.sum 

puts "Part1: #{sum}"