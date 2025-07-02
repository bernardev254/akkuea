package api

import (
	"gin/models"
	"gin/services"

	"github.com/gin-gonic/gin"
)

func PingHandler(c *gin.Context) {
	response := models.PingResponse{Message: services.GetPingMessage()}
	c.JSON(200, response)
}
