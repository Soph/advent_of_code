require "set"

file = File.open(ARGV[0])
data = file.readlines.map(&:chomp).join("\n").split("\n\n")

available_colors = data[0].split(", ").sort{|color_a, color_b| color_b.size <=> color_a.size }
patterns = data[1].split("\n")

def find_match(pattern, available_colors, seen = {})
  count = 0
  available_colors.each do |color|
    if pattern =~ /^#{color}/
      remaining = pattern.gsub(/^#{color}/, "")
      if remaining == ""
        count += 1
      elsif !seen[remaining].nil?
        count += seen[remaining]
      else
        sub_count = find_match(remaining, available_colors, seen)
        seen[remaining] = sub_count
        count += sub_count
      end
    end
  end
  return count
end

possible = 0
counts = []
patterns.each do |pattern|
  counts << find_match(pattern, available_colors)
end

puts "Part1: #{counts.select{|count| count > 0}.size}"
puts "Part2: #{counts.sum}"