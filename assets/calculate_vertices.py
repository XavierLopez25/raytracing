def calculate_square_corners(grid_columns, grid_rows, texture_width, texture_height):
    column_width = texture_width // grid_columns
    row_height = texture_height // grid_rows
    square_coordinates = {}
    
    # Calculate the corners for each grid square and store in a dictionary
    for row in range(grid_rows):
        for col in range(grid_columns):
            # Each square's number
            square_number = row * grid_columns + col + 1
            
            # Corners of the square
            x1 = col * column_width
            y1 = row * row_height
            x2 = x1 + column_width
            y2 = y1 + row_height
            
            # Store coordinates for each square
            square_coordinates[square_number] = [
                (x1, y1), # Top-left
                (x2, y1), # Top-right
                (x1, y2), # Bottom-left
                (x2, y2)  # Bottom-right
            ]
    
    return square_coordinates

# Configuration
texture_width = 500
texture_height = 375
grid_columns = 3
grid_rows = 4

# Get coordinates
square_coords = calculate_square_corners(grid_columns, grid_rows, texture_width, texture_height)

# Squares of interest
interest_squares = [2, 4, 5, 6, 8, 11]
for square in interest_squares:
    corners = square_coords[square]
    print(f"Face {square}: {corners[0]} {corners[1]} {corners[2]} {corners[3]}")
