package api

import (
	"net/http"

	"gin/config"
	"gin/models"

	"github.com/gin-gonic/gin"
)

type UsersResponse struct {
	Data    []models.User `json:"data"`
	Count   int           `json:"count"`
	Message string        `json:"message"`
}

type ErrorResponse struct {
	Error   string `json:"error"`
	Message string `json:"message"`
}

func GetAllUsers(c *gin.Context) {
	db := config.GetDB()
	
	var users []models.User
	
	if err := db.Find(&users).Error; err != nil {
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:   "database_error",
			Message: "Failed to retrieve users from database",
		})
		return
	}

	c.JSON(http.StatusOK, UsersResponse{
		Data:    users,
		Count:   len(users),
		Message: "Users retrieved successfully",
	})
}

func CreateUser(c *gin.Context) {
	db := config.GetDB()
	
	var user models.User
	
	if err := c.ShouldBindJSON(&user); err != nil {
		c.JSON(http.StatusBadRequest, ErrorResponse{
			Error:   "invalid_input",
			Message: "Invalid user data provided",
		})
		return
	}

	if err := db.Create(&user).Error; err != nil {
		c.JSON(http.StatusInternalServerError, ErrorResponse{
			Error:   "database_error",
			Message: "Failed to create user in database",
		})
		return
	}

	c.JSON(http.StatusCreated, gin.H{
		"data":    user,
		"message": "User created successfully",
	})
}