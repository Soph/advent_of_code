file = File.open(ARGV[0])
lines = file.readlines.map(&:chomp)

records = lines.map {|line| line.split(" ") }
records = records.map {|record| [record[0].chars, record[1].split(",").map(&:to_i)]}

CACHE = {}
def check(record, groups)
  result = CACHE[[record.size, groups.size]]
  return result if result

  return 0 if record.size < (groups.size - 1 + groups.sum)
  
  if groups.empty? && record.select{|c| c == '#'}.empty?
    return 1
  end

  if groups.empty? || record.nil?
    return 0
  end

  if record[0] == '.'
    return check(record[1..], groups)
  end

  if record[0] == '?'
    a = check(record[1..].insert(0,'#'), groups)
    b = check(record[1..].insert(0,'.'), groups)
    CACHE[[record.size, groups.size]] = a+b
    return a+b
  end

  if record[0] == '#'
    group = groups[0]
    if record[0...group].select {|c| c == '.'}.empty? # matches
      if record[group] == '#' # not possible        
        return 0
      end

      new_record = record[(group+1)..]
      new_record ||= []
      return check(new_record, groups[1..])
    else
      return 0
    end
  end

  0
end

sum = 0
records.each do |record|
  CACHE = {}
  new_record = ([record[0].join]*5).join("?").chars
  new_groups = ([record[1].join(",")]*5).join(",").split(",").map(&:to_i)
  sum += check(new_record, new_groups)
end
puts sum
