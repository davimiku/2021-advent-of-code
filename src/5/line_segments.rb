# Represents an x,y location
Point = Struct.new(:x, :y)

# Represents two points connected together
class Segment
  def initialize(first, second)
    @first = first
    @second = second

    @points = calculate_points
  end

  def to_s
    "(#{@first[:x]}, #{@first[:y]}) -> (#{@second[:x]}, #{@second[:y]})"
  end

  # return an Array of Point for all the Points between @first and @second
  def calculate_points
    points = []
    # figure out if points have the same x or the same y
    if @first[:x] == @second[:x]
      x = @first[:x]
      # vertical line

      (@second[:y]..@first[:y]).each { |n| points.push(Point.new(x, n)) } if @first[:y] > @second[:y]
      (@first[:y]..@second[:y]).each { |n| points.push(Point.new(x, n)) } if @second[:y] > @first[:y]

    elsif @first[:y] == @second[:y]
      y = @first[:y]
      # horizontal line

      (@second[:x]..@first[:x]).each { |n| points.push(Point.new(n, y)) } if @first[:x] > @second[:x]
      (@first[:x]..@second[:x]).each { |n| points.push(Point.new(n, y)) } if @second[:x] > @first[:x]

    end

    points
  end
end

class Board
  def initialize(size)
    @cols = Array.new(size) { Array.new(size, 0) }
  end

  attr_reader :cols

  def to_s
    output = ''
    len = @cols.length
    (0..len - 1).each do |y|
      (0..len - 1).each do |x|
        output += @cols[x][y].to_s
        output += ', '
      end
      output += "\n"
    end
    output
  end
end

# Array of strings
lines = IO.readlines('src/5/data.txt', chomp: true)

# Array of ["457,597", "61,597"]
split_lines = lines.map { |line| line.split(' -> ') }

split_points = split_lines.map do |split_line|
  # split_line is ["457,597", "61,597"]

  points = split_line.map do |str|
    # nums: [457, 597]
    nums = str.split(',').map { |s| Integer(s) }

    Point.new(nums[0], nums[1])
  end

  # points: [Point(457, 597), Point(61,597)]
  points
end

segments = split_points.map { |points| Segment.new(points[0], points[1]) }
board = Board.new(1000)

total = 0

segments.each do |segment|
  points = segment.calculate_points

  points.each do |point|
    # puts "(#{point[:x]},#{point[:y]})"
    board.cols[point[:x]][point[:y]] += 1

    total += 1 if board.cols[point[:x]][point[:y]] >= 2
  end
end

puts "total: #{total}"
