require "set"

file = File.open(ARGV[0])
sections = file.read.split("\n\n")

class AlmanacRange
  def initialize(start, count)
    @start = start
    @count = count
    @range = (@start...(@start+@count))
  end

  def include?(number)
    @range.include?(number)
  end

  def offset(number)
    number - @start
  end

  def to_s
    @range.to_s
  end
end

current = sections[0].split(": ").last.split(" ").map(&:to_i).to_set
sections[1..-1].each do |section|
  mapped = Set.new
  section.split(":\n").last.split("\n").each do |mapping|
    map = mapping.split(" ").map(&:to_i)
    from = AlmanacRange.new(map[1], map[2])
    current.each do |item|
      if from.include?(item)
        mapped << (map[0] + from.offset(item))
        current.delete(item)
      end
    end
  end
  current = mapped.merge(current)
end

puts "Part1: #{current.min}"

current = Set.new
seed_ranges = sections[0].split(": ").last.split(" ").map(&:to_i)
seed_ranges.each_slice(2) do |first, length|
  current << (first...(first+length))
end
sections[1..-1].each do |section|
  #puts section
  mapped = Set.new
  section.split(":\n").last.split("\n").each do |mapping|
    map = mapping.split(" ").map(&:to_i)
    from = (map[1]...(map[1]+map[2]))
    to = (map[0]...(map[0]+map[2]))
    left = Set.new
    current.each do |item|
      offset = to.first - from.first # dest - source
      if from.include?(item.first) && from.include?(item.last) # fits inside
        mapped << ((item.first+offset)...(item.end+offset))
        current.delete(item)
      elsif from.include?(item.first)
        # first is in, but last is not, so we can cover all til end of "to" map
        new_mapped = ((item.first+offset)...to.end)
        # add length of those we could map, keep rest
        rest = ((item.first + new_mapped.size)...item.end)
        current.delete(item)
        mapped << new_mapped
        left << rest
      elsif from.include?(item.last)
        # last is in, but first is not, so first til beginning of map stays, rest gets mapped
        rest = (item.first...from.first)
        new_mapped = (from.first+offset...(item.end+offset))
        current.delete(item)
        mapped << new_mapped
        left << rest
      end
    end
    current.merge(left)
  end
  current = mapped.merge(current)
end

puts "Part2: #{current.min_by {|range| range.first }.first}"
