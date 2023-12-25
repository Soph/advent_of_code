require "Set"

file = File.open(ARGV[0])
line = file.readlines.map(&:chomp)
