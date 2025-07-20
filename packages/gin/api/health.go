package api

import (
	"gin/config"
	"net/http"

	"github.com/gin-gonic/gin"
)

type HealthResponse struct {
	Status   string `json:"status"`
	Database string `json:"database"`
	Message  string `json:"message"`
}

func HealthHandler(c *gin.Context) {
	db := config.GetDB()
	
	// Test database connection
	sqlDB, err := db.DB()
	if err != nil {
		c.JSON(http.StatusInternalServerError, HealthResponse{
			Status:   "error",
			Database: "disconnected",
			Message:  "Failed to get database instance",
		})
		return
	}

	if err := sqlDB.Ping(); err != nil {
		c.JSON(http.StatusInternalServerError, HealthResponse{
			Status:   "error",
			Database: "disconnected",
			Message:  "Database ping failed",
		})
		return
	}

	c.JSON(http.StatusOK, HealthResponse{
		Status:   "healthy",
		Database: "connected",
		Message:  "All systems operational",
	})
}