file = File.open(ARGV[0])
data = {}
file.readlines.map(&:chomp).map do |l|
  part = l.split(': ')
  data[part[0]] = part[1].split(' ')
end

def find_path(data, start, finish)
  paths = Set.new
  paths << [start]
  done = Set.new
  loop do
    new_paths = Set.new
    paths.each do |path|
      if data[path[-1]].nil?
        puts "Dead End: #{path}"
        next
      end
      data[path[-1]].each do |n|
        new_path = path + [n]
        if n == finish
          puts "Done: #{new_path}"
          done << new_path
        else
          new_paths << new_path
          puts "Continue: #{new_path}"
        end
      end
    end
    break if new_paths.empty?

    paths = new_paths
  end
  done
end

reverse_data = {}
data.each do |key, values|
  values.each do |value|
    reverse_data[value] ||= []
    reverse_data[value] << key
  end
end

puts find_path(reverse_data, 'dac', 'fft').inspect
exit

srv_dac = find_path(data, 'svr', 'dac')
srv_fft = find_path(data, 'svr', 'fft')
dac_fft = find_path(data, 'dac', 'fft')
fft_dac = find_path(data, 'fft', 'dac')
dac_out = find_path(data, 'dac', 'out')
fft_out = find_path(data, 'fft', 'out')

puts "srv->dac: #{srv_dac}"
puts "srv->fft: #{srv_fft}"
puts "dac->fft: #{dac_fft}"
puts "fft->dac: #{fft_dac}"
puts "dac->out: #{dac_out}"
puts "fft->out: #{fft_out}"

exit
dac_fft = find_path(data, 'svr', 'dac') * find_path(data, 'dac', 'fft') * find_path(data, 'fft', 'out')
puts "via dac then fft: #{dac_fft}"
fft_dac = find_path(data, 'svr', 'fft') * find_path(data, 'fft', 'dac') * find_path(data, 'dac', 'out')
puts "Part 2: #{dac_fft} + #{fft_dac} = #{dac_fft + fft_dac}"
